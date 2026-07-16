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
use vec3::{Color, Vec3};

fn main() {
    println!("rt ray tracer — foundation ready (RT-001, RT-002, RT-003)");

    // Quick sanity check of math + camera (not PPM output yet).
    let v = Vec3::new(1.0, 2.0, 3.0).normalize();
    let ray = ray::Ray::new(Vec3::ZERO, v);
    let _ = ray.at(1.0);
    let _color = Color::WHITE.clamp().to_rgb8();
    let _bg = Color::BLACK.to_rgb8();

    let cam = Camera::look_at(
        Vec3::new(0.0, 1.0, 4.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        4.0 / 3.0,
    );
    let center = cam.get_ray(0.5, 0.5);
    let _ = (cam.eye(), cam.forward(), center);
}
