mod camera;
mod light;
mod material;
mod objects;
mod ppm;
mod ray;
mod scene;
mod scenes;
mod tracer;
mod vec3;

use ppm::write_ppm_p3;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process;
use tracer::{render_frame_with, TraceOptions};

/// Defaults for quick local previews. Use `--width 800 --height 600` for audit images.
const DEFAULT_WIDTH: u32 = 400;
const DEFAULT_HEIGHT: u32 = 300;
const DEFAULT_MAX_DEPTH: u32 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SceneId {
    /// RT-011 — sphere + ground, Scene 1 brightness.
    Scene1,
    /// RT-012 — plane + cube, dimmer light.
    Scene2,
    /// RT-013 — all four objects, front camera.
    Scene3,
    /// RT-014 — same world as Scene 3, alternate camera.
    Scene4,
    /// RT-016 — metal sphere demo (use with `--reflection`).
    Reflection,
}

struct Args {
    width: u32,
    height: u32,
    scene: SceneId,
    /// If set, write to this path; otherwise write PPM to stdout.
    output: Option<String>,
    reflections: bool,
    max_depth: u32,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    let mut scene = SceneId::Scene1;
    let mut output = None;
    let mut reflections = false;
    let mut max_depth = DEFAULT_MAX_DEPTH;

    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--width" | "-w" => {
                i += 1;
                width = parse_dim(argv.get(i), "--width")?;
            }
            "--height" => {
                i += 1;
                height = parse_dim(argv.get(i), "--height")?;
            }
            "--scene" | "-s" => {
                i += 1;
                scene = parse_scene(argv.get(i))?;
            }
            "--output" | "-o" => {
                i += 1;
                let path = argv
                    .get(i)
                    .ok_or_else(|| "missing value for --output".to_string())?;
                output = Some(path.clone());
            }
            "--reflection" | "-r" => {
                reflections = true;
            }
            "--max-depth" => {
                i += 1;
                max_depth = parse_dim(argv.get(i), "--max-depth")?;
            }
            "--help" => {
                print_usage();
                process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
        i += 1;
    }

    Ok(Args {
        width,
        height,
        scene,
        output,
        reflections,
        max_depth,
    })
}

fn parse_dim(value: Option<&String>, flag: &str) -> Result<u32, String> {
    let raw = value.ok_or_else(|| format!("missing value for {flag}"))?;
    let n: u32 = raw
        .parse()
        .map_err(|_| format!("invalid {flag} value: {raw}"))?;
    if n == 0 {
        return Err(format!("{flag} must be > 0"));
    }
    Ok(n)
}

fn parse_scene(value: Option<&String>) -> Result<SceneId, String> {
    let raw = value.ok_or_else(|| "missing value for --scene".to_string())?;
    match raw.as_str() {
        "1" | "scene1" | "sphere" => Ok(SceneId::Scene1),
        "2" | "scene2" | "plane-cube" | "cube" => Ok(SceneId::Scene2),
        "3" | "scene3" | "all" => Ok(SceneId::Scene3),
        "4" | "scene4" | "alt" | "alt-camera" => Ok(SceneId::Scene4),
        "5" | "reflection" | "metal" => Ok(SceneId::Reflection),
        other => Err(format!(
            "unknown scene '{other}' (try: 1 / sphere, 2 / cube, 3 / all, 4 / alt, 5 / reflection)"
        )),
    }
}

fn print_usage() {
    eprintln!(
        "Usage: rt [--scene ID] [--width N] [--height N] [--output FILE] [-r] [--max-depth N]\n\
         \n\
         Scenes:\n\
           1 | sphere       Scene 1 — sphere only (RT-011)\n\
           2 | cube         Scene 2 — plane + cube, dimmer light (RT-012)\n\
           3 | all          Scene 3 — all four objects (RT-013)\n\
           4 | alt          Scene 4 — same as 3, alternate camera (RT-014)\n\
           5 | reflection   Bonus metal-sphere demo (RT-016; use with -r)\n\
         \n\
         Bonus:\n\
           -r | --reflection   Enable recursive reflections\n\
           --max-depth N       Max bounce depth when -r is set (default {DEFAULT_MAX_DEPTH})\n\
         \n\
         Defaults: scene 1, {DEFAULT_WIDTH}×{DEFAULT_HEIGHT} (dev). Audit size: 800×600.\n\
         Examples:\n\
           cargo run -- --scene 3 --width 800 --height 600 -o scenes/scene3_all.ppm\n\
           cargo run --release -- -s reflection -r --width 800 --height 600 -o scenes/scene_reflection.ppm\n\
         Without --output, writes a P3 PPM to stdout."
    );
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            print_usage();
            process::exit(1);
        }
    };

    let aspect = args.width as f64 / args.height as f64;
    let (scene, camera) = match args.scene {
        SceneId::Scene1 => scenes::scene1_sphere(aspect),
        SceneId::Scene2 => scenes::scene2_plane_cube(aspect),
        SceneId::Scene3 => scenes::scene3_all(aspect),
        SceneId::Scene4 => scenes::scene4_alt_camera(aspect),
        SceneId::Reflection => scenes::scene_reflection_demo(aspect),
    };

    let scene_label = match args.scene {
        SceneId::Scene1 => "scene1_sphere",
        SceneId::Scene2 => "scene2_plane_cube",
        SceneId::Scene3 => "scene3_all",
        SceneId::Scene4 => "scene4_alt_camera",
        SceneId::Reflection => "scene_reflection",
    };

    let opts = if args.reflections {
        TraceOptions::with_reflections(args.max_depth)
    } else {
        TraceOptions::default()
    };

    eprintln!(
        "rt: {scene_label} {}×{} reflections={} depth={} → {}",
        args.width,
        args.height,
        opts.reflections,
        opts.max_depth,
        args.output.as_deref().unwrap_or("stdout")
    );

    let pixels = render_frame_with(&scene, &camera, args.width, args.height, &opts);

    let result = match &args.output {
        Some(path) => {
            if let Some(parent) = std::path::Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        eprintln!("error: cannot create directory {}: {e}", parent.display());
                        process::exit(1);
                    }
                }
            }
            let mut file = match File::create(path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error: cannot create {path}: {e}");
                    process::exit(1);
                }
            };
            write_ppm_p3(&mut file, args.width, args.height, &pixels).and_then(|_| file.flush())
        }
        None => {
            let mut stdout = io::stdout().lock();
            write_ppm_p3(&mut stdout, args.width, args.height, &pixels).and_then(|_| stdout.flush())
        }
    };

    if let Err(e) = result {
        eprintln!("error: failed to write PPM: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod arg_tests {
    use super::*;

    #[test]
    fn defaults() {
        let args = parse_args(&["rt".into()]).unwrap();
        assert_eq!(args.width, DEFAULT_WIDTH);
        assert_eq!(args.height, DEFAULT_HEIGHT);
        assert_eq!(args.scene, SceneId::Scene1);
        assert!(args.output.is_none());
        assert!(!args.reflections);
    }

    #[test]
    fn scene_and_audit_size() {
        let args = parse_args(&[
            "rt".into(),
            "--scene".into(),
            "1".into(),
            "--width".into(),
            "800".into(),
            "--height".into(),
            "600".into(),
            "-o".into(),
            "scenes/scene1_sphere.ppm".into(),
        ])
        .unwrap();
        assert_eq!(args.scene, SceneId::Scene1);
        assert_eq!(args.width, 800);
        assert_eq!(args.height, 600);
        assert_eq!(args.output.as_deref(), Some("scenes/scene1_sphere.ppm"));
    }

    #[test]
    fn scene_aliases() {
        assert_eq!(
            parse_args(&["rt".into(), "-s".into(), "sphere".into()])
                .unwrap()
                .scene,
            SceneId::Scene1
        );
        assert_eq!(
            parse_args(&["rt".into(), "-s".into(), "2".into()])
                .unwrap()
                .scene,
            SceneId::Scene2
        );
        assert_eq!(
            parse_args(&["rt".into(), "-s".into(), "all".into()])
                .unwrap()
                .scene,
            SceneId::Scene3
        );
        assert_eq!(
            parse_args(&["rt".into(), "-s".into(), "4".into()])
                .unwrap()
                .scene,
            SceneId::Scene4
        );
        assert_eq!(
            parse_args(&["rt".into(), "-s".into(), "reflection".into()])
                .unwrap()
                .scene,
            SceneId::Reflection
        );
    }

    #[test]
    fn reflection_flags() {
        let args = parse_args(&[
            "rt".into(),
            "-r".into(),
            "--max-depth".into(),
            "8".into(),
            "-s".into(),
            "5".into(),
        ])
        .unwrap();
        assert!(args.reflections);
        assert_eq!(args.max_depth, 8);
        assert_eq!(args.scene, SceneId::Reflection);
    }
}
