//! Scene container: objects + lights (shared by RT-008 / RT-009).

use crate::light::Light;
use crate::objects::{Cube, Cylinder, HitRecord, Hittable, Plane, Sphere};
use crate::ray::Ray;
use crate::vec3::Color;

/// One scene primitive; enum keeps the world heap-free and object-safe-free.
#[derive(Clone, Debug)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Self::Sphere(o) => o.hit(ray, t_min, t_max),
            Self::Plane(o) => o.hit(ray, t_min, t_max),
            Self::Cube(o) => o.hit(ray, t_min, t_max),
            Self::Cylinder(o) => o.hit(ray, t_min, t_max),
        }
    }
}

/// World state for tracing: hittables, lights, and ambient level.
#[derive(Clone, Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    /// Ambient term passed to Lambertian shading (typically ~0.05–0.1).
    pub ambient: f64,
    pub background: Color,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            ambient: crate::light::DEFAULT_AMBIENT,
            background: Color::new(0.55, 0.7, 0.95),
        }
    }

    pub fn add(&mut self, object: Object) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn add_light(&mut self, light: Light) -> &mut Self {
        self.lights.push(light);
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient.max(0.0);
        self
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    /// Closest intersection in `[t_min, t_max]`.
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut best: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest) {
                closest = hit.t;
                best = Some(hit);
            }
        }

        best
    }

    /// `true` if any object blocks the ray before `t_max` (shadow query).
    pub fn is_occluded(&self, ray: &Ray, t_max: f64) -> bool {
        self.objects
            .iter()
            .any(|o| o.hit(ray, crate::light::SHADOW_BIAS, t_max).is_some())
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn closest_hit_wins() {
        let mut scene = Scene::new();
        scene
            .add(Object::Sphere(Sphere::with_albedo(
                Vec3::new(0.0, 0.0, -5.0),
                1.0,
                Color::new(1.0, 0.0, 0.0),
            )))
            .add(Object::Sphere(Sphere::with_albedo(
                Vec3::new(0.0, 0.0, -3.0),
                0.5,
                Color::new(0.0, 1.0, 0.0),
            )));

        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));
        let hit = scene.hit(&ray, 0.001, f64::INFINITY).unwrap();
        // Nearer sphere is at z=-3, radius 0.5 → front at z=-2.5 → t=2.5
        assert!((hit.t - 2.5).abs() < 1e-9);
        assert_eq!(hit.material.albedo, Color::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn occlusion_detects_blocker() {
        let mut scene = Scene::new();
        scene.add(Object::Sphere(Sphere::with_albedo(
            Vec3::new(0.0, 1.0, 0.0),
            0.5,
            Color::WHITE,
        )));

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert!(scene.is_occluded(&ray, 10.0));
        assert!(!scene.is_occluded(&ray, 0.1));
    }
}
