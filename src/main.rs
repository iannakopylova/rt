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
use scene::{Object, Scene};
use tracer::render_frame;
use vec3::{Color, Vec3};

fn main() {
    println!("rt ray tracer — core loop (RT-009)");

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
        1.0,
    );

    // Tiny preview frame — full PPM write is RT-010.
    let width = 8u32;
    let height = 8u32;
    let pixels = render_frame(&scene, &cam, width, height);
    let center = pixels[(height / 2 * width + width / 2) as usize];
    let corner = pixels[0];

    println!(
        "rendered {width}×{height} ({} pixels)",
        pixels.len()
    );
    println!(
        "center≈({:.3},{:.3},{:.3})  corner≈({:.3},{:.3},{:.3})",
        center.r, center.g, center.b, corner.r, corner.g, corner.b
    );
}
