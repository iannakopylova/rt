//! Texture sampling (RT-018 bonus): P3 PPM loader + albedo resolution.
//!
//! [`sample_albedo`] is the single point where a material's surface color is
//! resolved during shading (wired into `light::shade_lambertian` and the
//! reflection/refraction tints in `tracer::trace_recursive`). It is deliberately
//! the *only* place that knows about `-t`/`--textures`, texture files, and the
//! decode cache — everything else just asks "what color is this material at this
//! UV" and gets a `Color` back, textured or not.
//!
//! # Locked signature
//! The ticket pins `sample_albedo(material, uv) -> Color`. This project uses
//! `f64` everywhere (`Vec3`, `Color`, `Ray`, `Camera`, …), so `uv` is `(f64, f64)`
//! rather than `(f32, f32)` to match — the ticket explicitly allows adjusting
//! argument types to fit the existing types.

use crate::material::Material;
use crate::vec3::Color;
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// Decoded texture: row-major pixels (row 0 = first scanline in the PPM file).
#[derive(Debug)]
pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Texture {
    /// Nearest-neighbor sample; `u`/`v` outside `[0, 1]` wrap (tiles) via `rem_euclid`.
    pub fn sample(&self, u: f64, v: f64) -> Color {
        if self.width == 0 || self.height == 0 {
            return Color::BLACK;
        }
        let uu = u.rem_euclid(1.0);
        let vv = v.rem_euclid(1.0);
        let x = ((uu * self.width as f64) as usize).min(self.width - 1);
        let y = ((vv * self.height as f64) as usize).min(self.height - 1);
        self.pixels[y * self.width + x]
    }
}

/// Parse an ASCII **P3** PPM: `P3` magic, `width height`, `maxval`, then `width*height`
/// `R G B` triples. `#` starts a comment that runs to end of line, per the PPM spec.
pub fn load_ppm_p3(path: &str) -> Result<Texture, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let cleaned: String = text
        .lines()
        .map(|line| line.split('#').next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n");
    let mut tokens = cleaned.split_whitespace();

    let magic = tokens.next().ok_or_else(|| format!("{path}: empty file"))?;
    if magic != "P3" {
        return Err(format!(
            "{path}: unsupported format '{magic}' (only P3 ASCII PPM is supported)"
        ));
    }

    let width = next_usize(&mut tokens, path, "width")?;
    let height = next_usize(&mut tokens, path, "height")?;
    let maxval = next_usize(&mut tokens, path, "maxval")?;
    if width == 0 || height == 0 {
        return Err(format!("{path}: width/height must be > 0"));
    }
    if maxval == 0 {
        return Err(format!("{path}: maxval must be > 0"));
    }

    let expected = width * height;
    let mut pixels = Vec::with_capacity(expected);
    for _ in 0..expected {
        let r = next_usize(&mut tokens, path, "pixel component")?;
        let g = next_usize(&mut tokens, path, "pixel component")?;
        let b = next_usize(&mut tokens, path, "pixel component")?;
        let scale = maxval as f64;
        pixels.push(Color::new(r as f64 / scale, g as f64 / scale, b as f64 / scale).clamp());
    }

    Ok(Texture {
        width,
        height,
        pixels,
    })
}

fn next_usize<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    path: &str,
    what: &str,
) -> Result<usize, String> {
    let raw = tokens
        .next()
        .ok_or_else(|| format!("{path}: missing {what}"))?;
    raw.parse()
        .map_err(|_| format!("{path}: invalid {what} '{raw}'"))
}

// -- Global switches & cache -------------------------------------------------
//
// `sample_albedo`'s signature is locked to `(material, uv)`, so `-t` enablement
// can't be threaded through as a parameter the way `TraceOptions` gates
// reflection/refraction. A process-wide switch (set once from `main` before
// rendering starts) is the simplest way to keep that signature while still
// letting the CLI flag turn texturing off entirely. The tracer is single-threaded
// (no `std::thread` anywhere in this crate), so a plain `AtomicBool` is enough.
static TEXTURES_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable or disable texture sampling globally. Call once from `main` before
/// rendering, based on the `-t` / `--textures` CLI flag.
pub fn set_textures_enabled(enabled: bool) {
    TEXTURES_ENABLED.store(enabled, Ordering::Relaxed);
}

fn textures_enabled() -> bool {
    TEXTURES_ENABLED.load(Ordering::Relaxed)
}

type CacheEntry = Option<Arc<Texture>>;

fn cache() -> &'static Mutex<HashMap<&'static str, CacheEntry>> {
    static CACHE: OnceLock<Mutex<HashMap<&'static str, CacheEntry>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Load-or-fetch the decoded texture for `path`, memoizing failures too so a bad
/// path warns exactly once instead of once per ray hit.
fn get_texture(path: &'static str) -> CacheEntry {
    let mut cache = cache().lock().unwrap();
    if let Some(entry) = cache.get(path) {
        return entry.clone();
    }
    let result = match load_ppm_p3(path) {
        Ok(texture) => Some(Arc::new(texture)),
        Err(e) => {
            eprintln!("warning: texture '{path}' could not be loaded ({e}); using solid color");
            None
        }
    };
    cache.insert(path, result.clone());
    result
}

/// Resolve a material's surface color at `uv` — the single point where shading
/// and reflection/refraction read a material's color (RT-018).
///
/// Falls back to `material.albedo` when: textures are disabled (no `-t`), the
/// material has no `texture_path`, or the texture failed to load.
pub fn sample_albedo(material: &Material, uv: (f64, f64)) -> Color {
    if !textures_enabled() {
        return material.albedo;
    }
    let Some(path) = material.texture_path else {
        return material.albedo;
    };
    match get_texture(path) {
        Some(texture) => texture.sample(uv.0, uv.1),
        None => material.albedo,
    }
}

/// `cargo test` runs `#[test]` fns on a thread pool, but [`TEXTURES_ENABLED`] is
/// process-wide. Any test (in this module or elsewhere, e.g. `tracer::tests`)
/// that calls [`set_textures_enabled`] must hold this lock first so two such
/// tests can't race each other's flag flips.
#[cfg(test)]
pub(crate) static TEST_ENABLE_LOCK: Mutex<()> = Mutex::new(());

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Color;
    use std::io::Write;
    use std::sync::atomic::AtomicU32;

    static NEXT_ID: AtomicU32 = AtomicU32::new(0);

    fn write_temp_ppm(name: &str, contents: &str) -> String {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!("rt_texture_test_{name}_{id}.ppm"));
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn parses_minimal_p3() {
        let path = write_temp_ppm("minimal", "P3\n2 1\n255\n255 0 0 0 255 0\n");
        let tex = load_ppm_p3(&path).unwrap();
        assert_eq!(tex.width, 2);
        assert_eq!(tex.height, 1);
        assert_eq!(tex.pixels[0], Color::new(1.0, 0.0, 0.0));
        assert_eq!(tex.pixels[1], Color::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn ignores_comments_and_extra_whitespace() {
        let path = write_temp_ppm(
            "comments",
            "P3\n# a comment\n2 2 # trailing comment\n255\n\
             255 255 255   0 0 0\n0 0 0   255 255 255\n",
        );
        let tex = load_ppm_p3(&path).unwrap();
        assert_eq!(tex.width, 2);
        assert_eq!(tex.height, 2);
        assert_eq!(tex.pixels[0], Color::WHITE);
        assert_eq!(tex.pixels[3], Color::WHITE);
    }

    #[test]
    fn rejects_non_p3_magic() {
        let path = write_temp_ppm("badmagic", "P6\n1 1\n255\n255 0 0\n");
        assert!(load_ppm_p3(&path).is_err());
    }

    #[test]
    fn missing_file_is_an_error_not_a_panic() {
        assert!(load_ppm_p3("/nonexistent/path/does-not-exist.ppm").is_err());
    }

    #[test]
    fn sample_wraps_out_of_range_uv() {
        let tex = Texture {
            width: 2,
            height: 1,
            pixels: vec![Color::new(1.0, 0.0, 0.0), Color::new(0.0, 1.0, 0.0)],
        };
        assert_eq!(tex.sample(0.25, 0.5), Color::new(1.0, 0.0, 0.0));
        assert_eq!(tex.sample(0.75, 0.5), Color::new(0.0, 1.0, 0.0));
        // Wrapped: 1.25 behaves like 0.25.
        assert_eq!(tex.sample(1.25, 0.5), Color::new(1.0, 0.0, 0.0));
        // Negative wraps too, rather than panicking on an out-of-range index.
        assert_eq!(tex.sample(-0.25, 0.5), Color::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn untextured_material_returns_albedo_regardless_of_flag() {
        let _guard = TEST_ENABLE_LOCK.lock().unwrap();
        let m = Material::solid(Color::new(0.2, 0.4, 0.6));
        set_textures_enabled(false);
        assert_eq!(sample_albedo(&m, (0.5, 0.5)), m.albedo);
        set_textures_enabled(true);
        assert_eq!(sample_albedo(&m, (0.5, 0.5)), m.albedo);
        set_textures_enabled(false);
    }

    #[test]
    fn disabled_flag_ignores_texture_even_when_present() {
        let _guard = TEST_ENABLE_LOCK.lock().unwrap();
        let m = Material::textured(Color::new(0.1, 0.1, 0.1), "textures/checker_red.ppm");
        set_textures_enabled(false);
        assert_eq!(sample_albedo(&m, (0.0, 0.0)), m.albedo);
    }

    #[test]
    fn missing_texture_file_falls_back_to_albedo_without_panicking() {
        let _guard = TEST_ENABLE_LOCK.lock().unwrap();
        set_textures_enabled(true);
        let m = Material::textured(Color::new(0.3, 0.3, 0.3), "textures/does-not-exist.ppm");
        let c = sample_albedo(&m, (0.5, 0.5));
        assert_eq!(c, m.albedo);
        set_textures_enabled(false);
    }
}
