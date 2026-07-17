mod camera;
mod light;
mod material;
mod objects;
mod ppm;
mod ray;
mod scene;
mod tracer;
mod vec3;

use camera::Camera;
use light::Light;
use objects::Sphere;
use ppm::write_ppm_p3;
use scene::{Object, Scene};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process;
use tracer::render_frame;
use vec3::{Color, Vec3};

/// Defaults for quick local previews. Use `--width 800 --height 600` for audit images.
const DEFAULT_WIDTH: u32 = 400;
const DEFAULT_HEIGHT: u32 = 300;

struct Args {
    width: u32,
    height: u32,
    /// If set, write to this path; otherwise write PPM to stdout.
    output: Option<String>,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    let mut output = None;

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
            "--output" | "-o" => {
                i += 1;
                let path = argv
                    .get(i)
                    .ok_or_else(|| "missing value for --output".to_string())?;
                output = Some(path.clone());
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
        output,
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

fn print_usage() {
    eprintln!(
        "Usage: rt [--width N] [--height N] [--output FILE]\n\
         \n\
         Defaults: {DEFAULT_WIDTH}×{DEFAULT_HEIGHT} (dev). Audit size: 800×600.\n\
         Without --output, writes a P3 PPM to stdout (e.g. cargo run > out.ppm)."
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

    let mut scene = Scene::new().with_ambient(0.08);
    scene
        .add(Object::Sphere(Sphere::with_albedo(
            Vec3::new(0.0, 0.0, -3.0),
            1.0,
            Color::new(1.0, 0.25, 0.2),
        )))
        .add_light(Light::scene1_key(Vec3::new(2.0, 3.0, 1.0)));

    let cam = Camera::look_at(
        Vec3::new(0.0, 0.5, 2.0),
        Vec3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        aspect,
    );

    eprintln!(
        "rt: rendering {}×{} → {}",
        args.width,
        args.height,
        args.output.as_deref().unwrap_or("stdout")
    );

    let pixels = render_frame(&scene, &cam, args.width, args.height);

    let result = match &args.output {
        Some(path) => {
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
        assert!(args.output.is_none());
    }

    #[test]
    fn custom_size_and_output() {
        let args = parse_args(&[
            "rt".into(),
            "--width".into(),
            "800".into(),
            "--height".into(),
            "600".into(),
            "-o".into(),
            "out.ppm".into(),
        ])
        .unwrap();
        assert_eq!(args.width, 800);
        assert_eq!(args.height, 600);
        assert_eq!(args.output.as_deref(), Some("out.ppm"));
    }
}
