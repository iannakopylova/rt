//! Surface appearance for shading (RT-008 + RT-016 reflection + RT-017 refraction).

use crate::vec3::Color;

/// Diffuse albedo with optional mirror reflectivity and dielectric IOR.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub albedo: Color,
    /// Fraction of the bounce that follows the mirror reflection ray when
    /// reflections are enabled in the tracer (`0.0` … `1.0`).
    pub reflectivity: f64,
    /// Absolute refractive index (air ≈ `1.0`). Values `> 1` enable glass-like
    /// refraction when `--refraction` is set (e.g. glass ≈ `1.5`).
    pub ior: f64,
    /// P3 PPM path sampled by [`crate::texture::sample_albedo`] for this material's
    /// surface color when textures are enabled (`-t`). `None` ⇒ always `albedo`.
    pub texture_path: Option<&'static str>,
}

impl Material {
    pub fn solid(albedo: Color) -> Self {
        Self {
            albedo,
            reflectivity: 0.0,
            ior: 1.0,
            texture_path: None,
        }
    }

    /// Metal / mirror: `reflectivity` blends Lambertian with recursive reflection.
    pub fn metal(albedo: Color, reflectivity: f64) -> Self {
        Self {
            albedo,
            reflectivity: reflectivity.clamp(0.0, 1.0),
            ior: 1.0,
            texture_path: None,
        }
    }

    /// Dielectric / glass: Snell's law + Fresnel when refraction is enabled.
    pub fn glass(albedo: Color, ior: f64) -> Self {
        Self {
            albedo,
            reflectivity: 0.0,
            ior: ior.max(1.0),
            texture_path: None,
        }
    }

    /// Solid material whose albedo is replaced by a sampled P3 PPM texture
    /// (RT-018) wherever [`crate::texture::sample_albedo`] resolves shading color.
    /// `albedo` remains as the fallback used when textures are disabled (no `-t`)
    /// or the file at `texture_path` fails to load.
    pub fn textured(albedo: Color, texture_path: &'static str) -> Self {
        Self {
            texture_path: Some(texture_path),
            ..Self::solid(albedo)
        }
    }

    /// Attach (or replace) a texture on an existing material, keeping its other
    /// properties — e.g. combine with [`Material::metal`] for a textured mirror.
    pub fn with_texture(mut self, texture_path: &'static str) -> Self {
        self.texture_path = Some(texture_path);
        self
    }

    /// True when this material should refract under [`crate::tracer::TraceOptions`].
    pub fn is_dielectric(self) -> bool {
        self.ior > 1.0 + 1e-9
    }
}
