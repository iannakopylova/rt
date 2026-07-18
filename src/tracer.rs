//! Ray tracer core loop (RT-009): closest hit, background, per-pixel render.
//! RT-016: optional recursive reflections behind [`TraceOptions`].
//! RT-017: optional dielectric refraction (Snell's law + Fresnel).

use crate::camera::Camera;
use crate::light::{shade_lambertian, SHADOW_BIAS};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::{Color, Vec3};

/// Controls recursive reflection / refraction. When both flags are false, all
/// materials shade as diffuse only (fast path for audit scenes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceOptions {
    pub reflections: bool,
    pub refractions: bool,
    pub max_depth: u32,
}

impl Default for TraceOptions {
    fn default() -> Self {
        Self {
            reflections: false,
            refractions: false,
            max_depth: 5,
        }
    }
}

impl TraceOptions {
    pub fn with_reflections(max_depth: u32) -> Self {
        Self {
            reflections: true,
            refractions: false,
            max_depth: max_depth.max(1),
        }
    }

    pub fn with_refractions(max_depth: u32) -> Self {
        Self {
            reflections: false,
            refractions: true,
            max_depth: max_depth.max(1),
        }
    }

    pub fn with_bounces(reflections: bool, refractions: bool, max_depth: u32) -> Self {
        Self {
            reflections,
            refractions,
            max_depth: max_depth.max(1),
        }
    }
}

/// Ideal specular reflection: `v` is the incoming direction (need not be unit).
pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    let n = normal.normalize();
    let dir = v.normalize();
    dir - n * (2.0 * dir.dot(n))
}

/// Snell's law. `uv` is the unit incident direction, `n` the unit shading normal
/// (facing against the incident ray). `etai_over_etat` is ηᵢ/ηₜ.
/// Returns `None` on total internal reflection.
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Option<Vec3> {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let k = 1.0 - r_out_perp.length_squared();
    if k < 0.0 {
        None
    } else {
        Some(r_out_perp + n * (-k.sqrt()))
    }
}

/// Schlick approximation for dielectric reflectance at a given cosine and ηᵢ/ηₜ.
pub fn schlick(cosine: f64, etai_over_etat: f64) -> f64 {
    let r0 = ((1.0 - etai_over_etat) / (1.0 + etai_over_etat)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

/// Trace a ray: background on miss, Lambertian (+ optional bounce) on hit.
pub fn trace(scene: &Scene, ray: &Ray) -> Color {
    trace_with(scene, ray, &TraceOptions::default())
}

pub fn trace_with(scene: &Scene, ray: &Ray, opts: &TraceOptions) -> Color {
    trace_recursive(scene, ray, opts, 0)
}

fn trace_recursive(scene: &Scene, ray: &Ray, opts: &TraceOptions, depth: u32) -> Color {
    match scene.hit(ray, 0.001, f64::INFINITY) {
        Some(hit) => {
            let local = shade_lambertian(&hit, &scene.lights, scene.ambient, |shadow, t_max| {
                scene.is_occluded(shadow, t_max)
            });

            if depth >= opts.max_depth {
                return local;
            }

            // Dielectric path (RT-017): refraction + Fresnel reflection at the interface.
            if opts.refractions && hit.material.is_dielectric() {
                let ior = hit.material.ior;
                let etai_over_etat = if hit.front_face { 1.0 / ior } else { ior };
                let unit_dir = ray.direction.normalize();
                let cos_theta = (-unit_dir).dot(hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = etai_over_etat * sin_theta > 1.0;
                let reflectance = if cannot_refract {
                    1.0
                } else {
                    schlick(cos_theta, etai_over_etat)
                };

                let reflected_dir = reflect(unit_dir, hit.normal);
                let reflect_origin = hit.point + hit.normal * SHADOW_BIAS;
                let reflected = trace_recursive(
                    scene,
                    &Ray::new(reflect_origin, reflected_dir),
                    opts,
                    depth + 1,
                );

                let transmitted = if cannot_refract {
                    Color::BLACK
                } else if let Some(refracted_dir) = refract(unit_dir, hit.normal, etai_over_etat)
                {
                    let refract_origin = hit.point - hit.normal * SHADOW_BIAS;
                    trace_recursive(
                        scene,
                        &Ray::new(refract_origin, refracted_dir),
                        opts,
                        depth + 1,
                    )
                } else {
                    Color::BLACK
                };

                // Tint by albedo; Fresnel blends reflection vs transmission.
                return (reflected * reflectance + transmitted * (1.0 - reflectance))
                    * hit.material.albedo;
            }

            let k = hit.material.reflectivity;
            if !opts.reflections || k <= 0.0 {
                return local;
            }

            let reflected_dir = reflect(ray.direction, hit.normal);
            // Bias along the normal so the bounce does not re-hit this surface.
            let origin = hit.point + hit.normal * SHADOW_BIAS;
            let bounce = Ray::new(origin, reflected_dir);
            let reflected = trace_recursive(scene, &bounce, opts, depth + 1);
            // Tint the mirror by albedo; blend with local diffuse.
            local * (1.0 - k) + (reflected * hit.material.albedo) * k
        }
        None => scene.background.color_for_ray(ray),
    }
}

/// Cast one camera ray through the center of pixel `(x, y)` and shade it.
pub fn trace_pixel(
    scene: &Scene,
    camera: &Camera,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Color {
    trace_pixel_with(scene, camera, x, y, width, height, &TraceOptions::default())
}

pub fn trace_pixel_with(
    scene: &Scene,
    camera: &Camera,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    opts: &TraceOptions,
) -> Color {
    let ray = camera.ray_through_pixel(x, y, width, height);
    trace_with(scene, &ray, opts)
}

/// Full primary-ray loop: row-major pixels, `y = 0` at the top (PPM order).
pub fn render_frame(scene: &Scene, camera: &Camera, width: u32, height: u32) -> Vec<Color> {
    render_frame_with(scene, camera, width, height, &TraceOptions::default())
}

pub fn render_frame_with(
    scene: &Scene,
    camera: &Camera,
    width: u32,
    height: u32,
    opts: &TraceOptions,
) -> Vec<Color> {
    let mut pixels = Vec::with_capacity((width as usize) * (height as usize));
    for y in 0..height {
        for x in 0..width {
            pixels.push(trace_pixel_with(scene, camera, x, y, width, height, opts));
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

        let center = pixels[(h / 2 * w + w / 2) as usize];
        assert!(center.r > 0.1);
        assert!(center.r > center.g);
        assert!(center.r > center.b);

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

    #[test]
    fn reflect_bounces_off_normal() {
        let v = Vec3::new(1.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let r = reflect(v, n).normalize();
        let expected = Vec3::new(1.0, 1.0, 0.0).normalize();
        assert!((r.dot(expected) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn refract_air_to_glass_bends_toward_normal() {
        // Incident from air into glass along a 45° path toward −Y on a +Y normal.
        let uv = Vec3::new(1.0, -1.0, 0.0).normalize();
        let n = Vec3::new(0.0, 1.0, 0.0);
        let eta = 1.0 / 1.5;
        let t = refract(uv, n, eta).unwrap().normalize();
        // Refracted ray should still go downward, but closer to the −normal (−Y).
        assert!(t.y < 0.0);
        assert!(t.y.abs() > uv.y.abs());
        assert!(t.x.abs() < uv.x.abs());
    }

    #[test]
    fn refract_total_internal_reflection_returns_none() {
        // Inside glass, grazing exit toward air (ηᵢ/ηₜ = 1.5): TIR.
        let uv = Vec3::new(0.95, 0.312249899, 0.0).normalize();
        let n = Vec3::new(0.0, -1.0, 0.0);
        assert!(refract(uv, n, 1.5).is_none());
    }

    #[test]
    fn metal_sphere_sees_sky_when_reflections_on() {
        let mut scene = Scene::new().with_background(Background::Solid(Color::new(0.1, 0.4, 0.9)));
        scene
            .add(Object::Plane(Plane::ground(
                -1.0,
                Material::solid(Color::new(0.3, 0.3, 0.3)),
            )))
            .add(Object::Sphere(Sphere::new(
                Vec3::new(0.0, 0.0, -3.0),
                1.0,
                Material::metal(Color::WHITE, 1.0),
            )))
            .add_light(Light::point(Vec3::new(2.0, 4.0, 1.0), Color::WHITE, 1.0));

        let cam = Camera::look_at(
            Vec3::new(0.0, 0.5, 2.0),
            Vec3::new(0.0, 0.0, -3.0),
            Vec3::new(0.0, 1.0, 0.0),
            50.0,
            1.0,
        );
        let ray = cam.get_ray(0.5, 0.55);
        let off = trace_with(&scene, &ray, &TraceOptions::default());
        let on = trace_with(&scene, &ray, &TraceOptions::with_reflections(4));
        // With reflections, the metal picks up blue sky; without, stay diffuse-darker.
        assert!(on.b > off.b);
    }

    #[test]
    fn glass_sphere_sees_colored_backdrop_when_refraction_on() {
        let mut scene = Scene::new().with_background(Background::Solid(Color::new(0.05, 0.05, 0.08)));
        scene
            .add(Object::Plane(Plane::ground(
                -1.0,
                Material::solid(Color::new(0.2, 0.2, 0.25)),
            )))
            // Bright red wall behind the glass so transmission is obvious.
            .add(Object::Plane(Plane::from_point_normal(
                Vec3::new(0.0, 0.0, -6.0),
                Vec3::new(0.0, 0.0, 1.0),
                Material::solid(Color::new(0.95, 0.15, 0.1)),
            )))
            .add(Object::Sphere(Sphere::new(
                Vec3::new(0.0, 0.0, -3.0),
                1.0,
                Material::glass(Color::WHITE, 1.5),
            )))
            .add_light(Light::point(Vec3::new(2.0, 5.0, 2.0), Color::WHITE, 1.2));

        let cam = Camera::look_at(
            Vec3::new(0.0, 0.3, 2.5),
            Vec3::new(0.0, 0.0, -3.0),
            Vec3::new(0.0, 1.0, 0.0),
            45.0,
            1.0,
        );
        let ray = cam.get_ray(0.5, 0.5);
        let off = trace_with(&scene, &ray, &TraceOptions::default());
        let on = trace_with(&scene, &ray, &TraceOptions::with_refractions(8));
        // With refraction, the glass transmits the red wall; without, diffuse only.
        assert!(on.r > off.r);
        assert!(on.r > on.g);
        assert!(on.r > on.b);
    }
}
