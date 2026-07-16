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
use material::Material;
use objects::{Cube, Cylinder, Hittable, Plane, Sphere};
use ray::Ray;
use vec3::{Color, Vec3};

fn main() {
    println!("rt ray tracer — foundation ready (RT-001 .. RT-007)");

    // Quick sanity check of math + camera + primitives (not PPM output yet).
    let v = Vec3::new(1.0, 2.0, 3.0).normalize();
    let ray = Ray::new(Vec3::ZERO, v);
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

    let sphere = Sphere::with_albedo(Vec3::new(0.0, 0.0, -5.0), 1.0, Color::new(1.0, 0.2, 0.2));
    if let Some(hit) = sphere.hit(
        &Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0)),
        0.001,
        f64::INFINITY,
    ) {
        let _ = (hit.t, hit.point, hit.normal, hit.front_face, hit.material);
    }

    let ground = Plane::ground(-1.0, Material::solid(Color::new(0.4, 0.4, 0.4)));
    let _wall = Plane::with_albedo(
        Vec3::new(0.0, 0.0, -10.0),
        Vec3::new(0.0, 0.0, 1.0),
        Color::new(0.8, 0.8, 0.9),
    );
    let _ = ground.hit(
        &Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
        0.001,
        f64::INFINITY,
    );

    let cube = Cube::with_albedo(Vec3::new(1.5, 0.0, -4.0), 1.0, Color::new(0.2, 0.5, 1.0));
    let _ = (
        cube.center(),
        cube.hit(
            &Ray::new(Vec3::ZERO, Vec3::new(0.3, 0.0, -1.0)),
            0.001,
            f64::INFINITY,
        ),
    );

    let cyl = Cylinder::with_albedo(Vec3::new(-1.5, 0.0, -4.0), 0.5, 2.0, Color::new(0.2, 0.8, 0.3));
    let _ = cyl.hit(
        &Ray::new(Vec3::ZERO, Vec3::new(-0.3, 0.0, -1.0)),
        0.001,
        f64::INFINITY,
    );
}
