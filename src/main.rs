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
use light::{Light, SCENE1_LIGHT_INTENSITY, SCENE2_LIGHT_INTENSITY};
use material::Material;
use objects::{Cube, Cylinder, Plane, Sphere};
use scene::{Object, Scene};
use tracer::trace;
use vec3::{Color, Vec3};

fn main() {
    println!("rt ray tracer — lighting & shadows (RT-008)");

    // Scene 1 style: bright key light.
    let mut bright = Scene::new().with_ambient(0.08);
    bright
        .add(Object::Sphere(Sphere::with_albedo(
            Vec3::new(0.0, 0.0, -5.0),
            1.0,
            Color::new(1.0, 0.2, 0.2),
        )))
        .add(Object::Plane(Plane::ground(
            -1.0,
            Material::solid(Color::new(0.4, 0.4, 0.4)),
        )))
        .add_light(Light::scene1_key(Vec3::new(2.0, 4.0, 2.0)));

    // Scene 2 style: same layout idea, lower brightness.
    let mut dim = Scene::new().with_ambient(0.08);
    dim.add(Object::Cube(Cube::with_albedo(
        Vec3::new(1.5, 0.0, -4.0),
        1.0,
        Color::new(0.2, 0.5, 1.0),
    )))
    .add(Object::Cylinder(Cylinder::with_albedo(
        Vec3::new(-1.5, 0.0, -4.0),
        0.5,
        2.0,
        Color::new(0.2, 0.8, 0.3),
    )))
    .add(Object::Plane(Plane::ground(
        -1.0,
        Material::solid(Color::new(0.4, 0.4, 0.4)),
    )))
    .add_light(Light::scene2_key(Vec3::new(2.0, 4.0, 2.0)));

    let cam = Camera::look_at(
        Vec3::new(0.0, 1.0, 4.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        4.0 / 3.0,
    );

    let center = cam.get_ray(0.5, 0.5);
    let c1 = trace(&bright, &center);
    let c2 = trace(&dim, &center);

    println!(
        "scene1 intensity={SCENE1_LIGHT_INTENSITY:.2} center≈({:.3},{:.3},{:.3})",
        c1.r, c1.g, c1.b
    );
    println!(
        "scene2 intensity={SCENE2_LIGHT_INTENSITY:.2} center≈({:.3},{:.3},{:.3})",
        c2.r, c2.g, c2.b
    );
}
