//! Ray tracing entry points — closest hit + shading (RT-009 core used by RT-008).

use crate::light::shade_lambertian;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Color;

/// Trace a primary (or recursive) ray: background on miss, Lambertian shade on hit.
pub fn trace(scene: &Scene, ray: &Ray) -> Color {
    match scene.hit(ray, 0.001, f64::INFINITY) {
        Some(hit) => shade_lambertian(&hit, &scene.lights, scene.ambient, |shadow, t_max| {
            scene.is_occluded(shadow, t_max)
        }),
        None => scene.background,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::{Light, SCENE1_LIGHT_INTENSITY, SCENE2_LIGHT_INTENSITY};
    use crate::material::Material;
    use crate::objects::{Plane, Sphere};
    use crate::scene::Object;
    use crate::vec3::Vec3;

    fn lit_ground_scene(intensity: f64) -> Scene {
        let mut scene = Scene::new().with_ambient(0.05);
        scene
            .add(Object::Plane(Plane::ground(
                0.0,
                Material::solid(Color::WHITE),
            )))
            .add(Object::Sphere(Sphere::with_albedo(
                Vec3::new(0.0, 1.0, 0.0),
                0.5,
                Color::new(1.0, 0.2, 0.2),
            )))
            .add_light(Light::point(
                Vec3::new(0.0, 5.0, 0.0),
                Color::WHITE,
                intensity,
            ));
        scene
    }

    #[test]
    fn miss_returns_background() {
        let scene = Scene::new().with_background(Color::new(0.1, 0.2, 0.3));
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(trace(&scene, &ray), Color::new(0.1, 0.2, 0.3));
    }

    #[test]
    fn hit_is_shaded_not_flat_albedo() {
        let scene = lit_ground_scene(SCENE1_LIGHT_INTENSITY);
        // Look at the lit top of the sphere.
        let ray = Ray::new(Vec3::new(0.0, 3.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let c = trace(&scene, &ray);
        assert!(c.r > 0.2);
        assert!(c.g < c.r);
    }

    #[test]
    fn scene2_brightness_darker_than_scene1() {
        let s1 = lit_ground_scene(SCENE1_LIGHT_INTENSITY);
        let s2 = lit_ground_scene(SCENE2_LIGHT_INTENSITY);
        let ray = Ray::new(Vec3::new(2.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));

        let c1 = trace(&s1, &ray);
        let c2 = trace(&s2, &ray);
        assert!(c1.r > c2.r);
    }

    #[test]
    fn ground_under_sphere_is_darker_than_open_ground() {
        let scene = lit_ground_scene(SCENE1_LIGHT_INTENSITY);

        // Straight down onto the ground under the sphere (in shadow).
        let shadowed = trace(
            &scene,
            &Ray::new(Vec3::new(0.0, 0.25, 0.0), Vec3::new(0.0, -1.0, 0.0)),
        );
        // Ground well away from the sphere (lit).
        let open = trace(
            &scene,
            &Ray::new(Vec3::new(3.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
        );

        assert!(open.r + open.g + open.b > shadowed.r + shadowed.g + shadowed.b);
    }
}
