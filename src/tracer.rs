//! Ray tracer core loop (RT-009): closest hit, background, per-pixel render.

use crate::camera::Camera;
use crate::light::shade_lambertian;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Color;

/// Trace a primary ray: background on miss, Lambertian shade on hit.
pub fn trace(scene: &Scene, ray: &Ray) -> Color {
    match scene.hit(ray, 0.001, f64::INFINITY) {
        Some(hit) => shade_lambertian(&hit, &scene.lights, scene.ambient, |shadow, t_max| {
            scene.is_occluded(shadow, t_max)
        }),
        None => scene.background.color_for_ray(ray),
    }
}

/// Cast one camera ray through the center of pixel `(x, y)` and shade it.
pub fn trace_pixel(scene: &Scene, camera: &Camera, x: u32, y: u32, width: u32, height: u32) -> Color {
    let ray = camera.ray_through_pixel(x, y, width, height);
    trace(scene, &ray)
}

/// Full primary-ray loop: row-major pixels, `y = 0` at the top (PPM order).
///
/// Returns `width * height` colors. PPM encoding is RT-010.
pub fn render_frame(scene: &Scene, camera: &Camera, width: u32, height: u32) -> Vec<Color> {
    let mut pixels = Vec::with_capacity((width as usize) * (height as usize));
    for y in 0..height {
        for x in 0..width {
            pixels.push(trace_pixel(scene, camera, x, y, width, height));
        }
    }
    pixels
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::{Light, SCENE1_LIGHT_INTENSITY, SCENE2_LIGHT_INTENSITY};
    use crate::material::Material;
    use crate::objects::{Plane, Sphere};
    use crate::scene::{Background, Object};
    use crate::vec3::Vec3;

    fn lit_ground_scene(intensity: f64) -> Scene {
        let mut scene = Scene::new()
            .with_ambient(0.05)
            .with_background(Background::Solid(Color::new(0.1, 0.1, 0.1)));
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

    fn front_camera() -> Camera {
        Camera::look_at(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        )
    }

    #[test]
    fn miss_returns_solid_background() {
        let scene = Scene::new().with_background(Background::Solid(Color::new(0.1, 0.2, 0.3)));
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(trace(&scene, &ray), Color::new(0.1, 0.2, 0.3));
    }

    #[test]
    fn sky_gradient_brighter_at_horizon_than_zenith_blue() {
        let sky = Background::default_sky();
        let up = Ray::new(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
        let down = Ray::new(Vec3::ZERO, Vec3::new(0.0, -1.0, 0.0));
        let zenith = sky.color_for_ray(&up);
        let horizon = sky.color_for_ray(&down);

        // Zenith is bluer; horizon is whiter.
        assert!(zenith.b > zenith.r);
        assert!(horizon.r > zenith.r);
    }

    #[test]
    fn hit_is_shaded_not_flat_albedo() {
        let scene = lit_ground_scene(SCENE1_LIGHT_INTENSITY);
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

        let shadowed = trace(
            &scene,
            &Ray::new(Vec3::new(0.0, 0.25, 0.0), Vec3::new(0.0, -1.0, 0.0)),
        );
        let open = trace(
            &scene,
            &Ray::new(Vec3::new(3.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
        );

        assert!(open.r + open.g + open.b > shadowed.r + shadowed.g + shadowed.b);
    }

    #[test]
    fn render_frame_size_and_center_hit() {
        let mut scene = Scene::new().with_background(Background::Solid(Color::BLACK));
        scene
            .add(Object::Sphere(Sphere::with_albedo(
                Vec3::new(0.0, 0.0, -3.0),
                1.0,
                Color::new(1.0, 0.0, 0.0),
            )))
            .add_light(Light::point(
                Vec3::new(2.0, 2.0, 0.0),
                Color::WHITE,
                1.0,
            ));

        let cam = front_camera();
        let w = 16u32;
        let h = 16u32;
        let pixels = render_frame(&scene, &cam, w, h);
        assert_eq!(pixels.len(), (w * h) as usize);

        // Center pixel should hit the red sphere (shaded, so still reddish).
        let center = pixels[(h / 2 * w + w / 2) as usize];
        assert!(center.r > 0.1);
        assert!(center.r > center.g);
        assert!(center.r > center.b);

        // Corner should miss → solid black background.
        assert_eq!(pixels[0], Color::BLACK);
    }

    #[test]
    fn trace_pixel_matches_manual_uv() {
        let scene = Scene::new().with_background(Background::Solid(Color::new(0.2, 0.3, 0.4)));
        let cam = front_camera();
        let via_helper = trace_pixel(&scene, &cam, 3, 5, 8, 8);
        let ray = cam.ray_through_pixel(3, 5, 8, 8);
        assert_eq!(via_helper, trace(&scene, &ray));
    }
}
