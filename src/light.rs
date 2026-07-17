//! Lights, Lambertian shading, and shadow rays (RT-008).

use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

/// Bias along the surface normal so shadow rays do not re-hit the same point.
pub const SHADOW_BIAS: f64 = 1e-4;

/// Small constant ambient so shadowed regions are not pure black.
pub const DEFAULT_AMBIENT: f64 = 0.08;

/// Default point-light intensity for Scene 1 (sphere).
pub const SCENE1_LIGHT_INTENSITY: f64 = 1.0;

/// Scene 2 must be dimmer than Scene 1 — use this (or lower) intensity.
pub const SCENE2_LIGHT_INTENSITY: f64 = 0.45;

/// A light source with configurable brightness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Light {
    /// Omnidirectional light at `position`. Intensity is not distance-attenuated
    /// (brightness is controlled only by `intensity`) so scene authors can tune
    /// look without fighting falloff.
    Point {
        position: Vec3,
        color: Color,
        intensity: f64,
    },
    /// Parallel rays; `direction` points **from the surface toward the light**
    /// (i.e. opposite the incoming light travel direction).
    Directional {
        direction: Vec3,
        color: Color,
        intensity: f64,
    },
}

impl Light {
    pub fn point(position: Vec3, color: Color, intensity: f64) -> Self {
        Self::Point {
            position,
            color,
            intensity: intensity.max(0.0),
        }
    }

    pub fn directional(direction: Vec3, color: Color, intensity: f64) -> Self {
        Self::Directional {
            direction: direction.normalize(),
            color,
            intensity: intensity.max(0.0),
        }
    }

    /// Scene 1 style: bright white point light above the subject.
    pub fn scene1_key(position: Vec3) -> Self {
        Self::point(position, Color::WHITE, SCENE1_LIGHT_INTENSITY)
    }

    /// Scene 2 style: same placement idea, lower brightness than Scene 1.
    pub fn scene2_key(position: Vec3) -> Self {
        Self::point(position, Color::WHITE, SCENE2_LIGHT_INTENSITY)
    }

    pub fn intensity(self) -> f64 {
        match self {
            Self::Point { intensity, .. } | Self::Directional { intensity, .. } => intensity,
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::Point { color, .. } | Self::Directional { color, .. } => color,
        }
    }

    /// Direction from `hit_point` toward the light, and max ray length for shadows.
    ///
    /// For directional lights, `t_max` is `f64::INFINITY`.
    pub fn sample(self, hit_point: Vec3) -> Option<(Vec3, f64)> {
        match self {
            Self::Point { position, .. } => {
                let to_light = position - hit_point;
                let dist = to_light.length();
                if dist < 1e-12 {
                    return None;
                }
                Some((to_light / dist, dist))
            }
            Self::Directional { direction, .. } => Some((direction, f64::INFINITY)),
        }
    }
}

/// Diffuse (Lambertian) shading with shadow rays.
///
/// `occluded(shadow_ray, t_max)` should return `true` when something blocks the light
/// before distance `t_max`.
pub fn shade_lambertian(
    hit: &HitRecord,
    lights: &[Light],
    ambient: f64,
    mut occluded: impl FnMut(&Ray, f64) -> bool,
) -> Color {
    let albedo = hit.material.albedo;
    let mut color = albedo * ambient.max(0.0);

    for light in lights {
        let intensity = light.intensity();
        if intensity <= 0.0 {
            continue;
        }

        let Some((to_light, t_max)) = light.sample(hit.point) else {
            continue;
        };

        let ndotl = hit.normal.dot(to_light);
        if ndotl <= 0.0 {
            continue;
        }

        let shadow_origin = hit.point + hit.normal * SHADOW_BIAS;
        let shadow_ray = Ray::new(shadow_origin, to_light);
        if occluded(&shadow_ray, t_max) {
            continue;
        }

        // Lambertian: albedo * light_color * intensity * cosθ
        color += albedo * light.color() * (intensity * ndotl);
    }

    color.clamp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::objects::{HitRecord, Hittable, Plane, Sphere};
    use crate::ray::Ray;

    fn front_hit() -> HitRecord {
        HitRecord {
            t: 1.0,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            front_face: true,
            material: Material::solid(Color::WHITE),
        }
    }

    #[test]
    fn scene2_is_dimmer_than_scene1() {
        assert!(SCENE2_LIGHT_INTENSITY < SCENE1_LIGHT_INTENSITY);
        let p = Vec3::new(2.0, 4.0, 2.0);
        assert!(Light::scene2_key(p).intensity() < Light::scene1_key(p).intensity());
    }

    #[test]
    fn intensity_scales_brightness() {
        let hit = front_hit();
        let bright = Light::point(Vec3::new(0.0, 0.0, 5.0), Color::WHITE, 1.0);
        let dim = Light::point(Vec3::new(0.0, 0.0, 5.0), Color::WHITE, 0.25);

        let c_bright = shade_lambertian(&hit, &[bright], 0.0, |_, _| false);
        let c_dim = shade_lambertian(&hit, &[dim], 0.0, |_, _| false);

        assert!(c_bright.r > c_dim.r * 2.0);
    }

    #[test]
    fn ambient_keeps_shadows_from_pure_black() {
        let hit = front_hit();
        let light = Light::point(Vec3::new(0.0, 0.0, 5.0), Color::WHITE, 1.0);
        let c = shade_lambertian(&hit, &[light], 0.1, |_, _| true);
        assert!(c.r > 0.0);
        assert!((c.r - 0.1).abs() < 1e-9);
    }

    #[test]
    fn back_facing_light_adds_nothing() {
        let hit = front_hit();
        // Light behind the surface (normal +Z, light at -Z).
        let light = Light::point(Vec3::new(0.0, 0.0, -5.0), Color::WHITE, 1.0);
        let c = shade_lambertian(&hit, &[light], 0.0, |_, _| false);
        assert_eq!(c, Color::BLACK);
    }

    #[test]
    fn shadow_ray_blocked_by_sphere() {
        let ground = Plane::ground(0.0, Material::solid(Color::WHITE));
        let blocker = Sphere::with_albedo(Vec3::new(0.0, 1.0, 0.0), 0.5, Color::new(1.0, 0.0, 0.0));
        let light = Light::point(Vec3::new(0.0, 5.0, 0.0), Color::WHITE, 1.0);

        // Point on the ground directly under the sphere — should be in shadow.
        let hit = ground
            .hit(
                &Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
                0.001,
                f64::INFINITY,
            )
            .unwrap();

        let lit = shade_lambertian(&hit, &[light], 0.0, |_, _| false);
        let shadowed = shade_lambertian(&hit, &[light], 0.0, |ray, t_max| {
            blocker.hit(ray, SHADOW_BIAS, t_max).is_some()
        });

        assert!(lit.r > 0.5);
        assert_eq!(shadowed, Color::BLACK);
    }

    #[test]
    fn directional_light_uses_infinite_range() {
        let light = Light::directional(Vec3::new(0.0, 1.0, 0.0), Color::WHITE, 1.0);
        let (dir, t_max) = light.sample(Vec3::ZERO).unwrap();
        assert!((dir.y - 1.0).abs() < 1e-9);
        assert!(t_max.is_infinite());
    }
}
