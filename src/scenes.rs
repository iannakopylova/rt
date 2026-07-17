//! Audit scene builders (RT-011+).

use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use crate::objects::{Cube, Plane, Sphere};
use crate::scene::{Object, Scene};
use crate::vec3::{Color, Vec3};

/// Scene 1 — sphere only (plus a ground plane so shadows are visible).
///
/// # Configurable knobs
/// Edit the constants below to move/resize the sphere or retarget the camera.
pub fn scene1_sphere(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let sphere_center = Vec3::new(0.0, 0.0, -4.0);
    let sphere_radius = 1.0;
    let ground_y = -1.0;
    let light_pos = Vec3::new(3.0, 5.0, 1.0);
    let eye = Vec3::new(0.0, 1.2, 3.0);
    let look_at = Vec3::new(0.0, 0.0, -4.0);
    let vfov_degrees = 50.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.08);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.55, 0.55, 0.58)),
        )))
        .add(Object::Sphere(Sphere::with_albedo(
            sphere_center,
            sphere_radius,
            Color::new(0.9, 0.25, 0.2),
        )))
        .add_light(Light::scene1_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

/// Scene 2 — plane + cube with **lower** light brightness than Scene 1.
///
/// Brightness: Scene 1 uses [`crate::light::SCENE1_LIGHT_INTENSITY`] (1.0);
/// this scene uses [`crate::light::SCENE2_LIGHT_INTENSITY`] (0.45).
pub fn scene2_plane_cube(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let cube_center = Vec3::new(0.0, 0.0, -4.0);
    let cube_edge = 1.6;
    let ground_y = -1.0;
    // Same placement as Scene 1 so the dimmer intensity is the clear difference.
    let light_pos = Vec3::new(3.0, 5.0, 1.0);
    let eye = Vec3::new(0.0, 1.5, 3.5);
    let look_at = Vec3::new(0.0, 0.0, -4.0);
    let vfov_degrees = 50.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.08);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.55, 0.55, 0.58)),
        )))
        .add(Object::Cube(Cube::with_albedo(
            cube_center,
            cube_edge,
            Color::new(0.25, 0.45, 0.9),
        )))
        .add_light(Light::scene2_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::{SCENE1_LIGHT_INTENSITY, SCENE2_LIGHT_INTENSITY};
    use crate::tracer::trace;

    #[test]
    fn scene1_has_sphere_and_ground() {
        let (scene, _) = scene1_sphere(4.0 / 3.0);
        assert_eq!(scene.objects.len(), 2);
        assert_eq!(scene.lights.len(), 1);
        assert!(matches!(scene.objects[0], Object::Plane(_)));
        assert!(matches!(scene.objects[1], Object::Sphere(_)));
    }

    #[test]
    fn scene1_uses_bright_key_light() {
        let (scene, _) = scene1_sphere(4.0 / 3.0);
        assert!((scene.lights[0].intensity() - SCENE1_LIGHT_INTENSITY).abs() < 1e-12);
    }

    #[test]
    fn scene1_center_ray_hits_sphere() {
        let (scene, cam) = scene1_sphere(4.0 / 3.0);
        let ray = cam.get_ray(0.5, 0.5);
        let hit = scene.hit(&ray, 0.001, f64::INFINITY).unwrap();
        let color = trace(&scene, &ray);
        assert!(hit.material.albedo.r > 0.5);
        assert!(color.r > color.g);
        assert!(color.r > color.b);
    }

    #[test]
    fn scene1_ground_shadow_darker_than_open_ground() {
        let (scene, _) = scene1_sphere(4.0 / 3.0);
        let under = trace(
            &scene,
            &crate::ray::Ray::new(Vec3::new(4.0, 0.5, -4.0), Vec3::new(-1.0, -0.4, 0.0)),
        );
        let open = trace(
            &scene,
            &crate::ray::Ray::new(Vec3::new(4.0, 0.5, -1.0), Vec3::new(-1.0, -0.4, 0.0)),
        );
        assert!(open.r + open.g + open.b > under.r + under.g + under.b);
    }

    #[test]
    fn scene2_has_plane_and_cube() {
        let (scene, _) = scene2_plane_cube(4.0 / 3.0);
        assert_eq!(scene.objects.len(), 2);
        assert!(matches!(scene.objects[0], Object::Plane(_)));
        assert!(matches!(scene.objects[1], Object::Cube(_)));
    }

    #[test]
    fn scene2_light_dimmer_than_scene1() {
        let (s1, _) = scene1_sphere(4.0 / 3.0);
        let (s2, _) = scene2_plane_cube(4.0 / 3.0);
        assert!(s2.lights[0].intensity() < s1.lights[0].intensity());
        assert!((s2.lights[0].intensity() - SCENE2_LIGHT_INTENSITY).abs() < 1e-12);
    }

    #[test]
    fn scene2_center_ray_hits_cube() {
        let (scene, cam) = scene2_plane_cube(4.0 / 3.0);
        let ray = cam.get_ray(0.5, 0.5);
        let hit = scene.hit(&ray, 0.001, f64::INFINITY).unwrap();
        let color = trace(&scene, &ray);
        assert!(hit.material.albedo.b > hit.material.albedo.r);
        assert!(color.b > 0.05);
    }

    #[test]
    fn scene2_ground_shadow_darker_than_open_ground() {
        let (scene, _) = scene2_plane_cube(4.0 / 3.0);
        let under = trace(
            &scene,
            &crate::ray::Ray::new(Vec3::new(4.0, 0.5, -4.0), Vec3::new(-1.0, -0.4, 0.0)),
        );
        let open = trace(
            &scene,
            &crate::ray::Ray::new(Vec3::new(4.0, 0.5, -1.0), Vec3::new(-1.0, -0.4, 0.0)),
        );
        assert!(open.r + open.g + open.b > under.r + under.g + under.b);
    }
}
