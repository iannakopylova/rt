//! Infinite flat plane (ground for Scenes 2–3).
//!
//! # Representation
//! Stored in Hessian form `normal · x + offset == 0` (unit `normal`).
//! Build from a point + normal, or use [`Plane::ground`] for a horizontal floor.
//!
//! # Offset height
//! Raise/lower the floor by changing `y` in `Plane::ground(y, …)`:
//! ```ignore
//! let floor = Plane::ground(-1.0, Material::solid(Color::new(0.4, 0.4, 0.4))); // y = -1
//! let raised = Plane::ground(0.0, Material::solid(Color::new(0.4, 0.4, 0.4)));  // y = 0
//! ```

use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

const PARALLEL_EPS: f64 = 1e-8;

/// World units per texture repeat, used by [`Plane::hit`] UV mapping (RT-018).
const DEFAULT_TILE_SIZE: f64 = 2.0;

/// Infinite plane used as ground / walls.
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    /// Unit surface normal (geometric “front”).
    pub normal: Vec3,
    /// Constant term so `normal · x + offset == 0` for every point on the plane.
    pub offset: f64,
    pub material: Material,
    /// World units per texture repeat along local X/Z (RT-018); see [`Plane::with_tile_size`].
    pub tile_size: f64,
}

impl Plane {
    /// Plane through `point` with the given (not necessarily unit) normal.
    pub fn from_point_normal(point: Vec3, normal: Vec3, material: Material) -> Self {
        let normal = normal.normalize();
        let offset = -normal.dot(point);
        Self {
            normal,
            offset,
            material,
            tile_size: DEFAULT_TILE_SIZE,
        }
    }

    /// Horizontal ground (`+Y` up) at world height `y`.
    pub fn ground(y: f64, material: Material) -> Self {
        Self::from_point_normal(Vec3::new(0.0, y, 0.0), Vec3::new(0.0, 1.0, 0.0), material)
    }

    pub fn with_albedo(point: Vec3, normal: Vec3, albedo: Color) -> Self {
        Self::from_point_normal(point, normal, Material::solid(albedo))
    }

    /// Override the texture repeat distance (world units per tile); e.g. a smaller
    /// value repeats a texture more densely across the plane.
    pub fn with_tile_size(mut self, tile_size: f64) -> Self {
        self.tile_size = if tile_size.abs() > 1e-9 {
            tile_size.abs()
        } else {
            DEFAULT_TILE_SIZE
        };
        self
    }

    /// Any point known to lie on the plane (useful for debugging / scene setup).
    #[allow(dead_code)] // handy for scene wiring / docs examples
    pub fn anchor(self) -> Vec3 {
        // Pick the point along the normal from the origin.
        self.normal * (-self.offset)
    }

    /// Tiled UV from world **X/Z** at the hit point (RT-018). Matches how this
    /// project only ever uses horizontal (`+Y`) ground planes; an arbitrarily
    /// tilted plane would still get a UV, just not a seam-free one.
    fn uv_at(self, point: Vec3) -> (f64, f64) {
        (
            (point.x / self.tile_size).rem_euclid(1.0),
            (point.z / self.tile_size).rem_euclid(1.0),
        )
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < PARALLEL_EPS {
            return None; // ray parallel to plane
        }

        // Solve normal · (origin + t·dir) + offset = 0
        let t = -(self.normal.dot(ray.origin) + self.offset) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        Some(HitRecord::from_outward_normal(
            t,
            point,
            self.normal,
            ray,
            self.material,
            self.uv_at(point),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn ground_at_height() {
        let plane = Plane::ground(-1.0, Material::solid(Color::new(0.5, 0.5, 0.5)));
        assert!(approx(plane.normal.y, 1.0));
        assert!(approx(plane.anchor().y, -1.0));
    }

    #[test]
    fn from_point_normal_normalizes() {
        let plane = Plane::with_albedo(
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Color::new(0.5, 0.5, 0.5),
        );
        assert!(approx(plane.normal.y, 1.0));
        assert!(approx(plane.normal.length(), 1.0));
    }

    #[test]
    fn hits_from_above() {
        let plane = Plane::ground(-1.0, Material::solid(Color::new(0.5, 0.5, 0.5)));
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, -1.0, 0.0));
        let hit = plane.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.t, 1.0));
        assert!(approx(hit.point.y, -1.0));
        assert!(approx(hit.normal.y, 1.0));
        assert!(hit.front_face);
    }

    #[test]
    fn uv_tiles_and_wraps_negative_coords() {
        // Default tile_size is 2.0: u = (x/2).rem_euclid(1), v = (z/2).rem_euclid(1).
        let plane = Plane::ground(-1.0, Material::solid(Color::new(0.5, 0.5, 0.5)));
        let ray = Ray::new(Vec3::new(3.0, 2.0, -1.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = plane.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.uv.0, 0.5)); // (3.0/2.0).rem_euclid(1.0)
        assert!(approx(hit.uv.1, 0.5)); // (-1.0/2.0).rem_euclid(1.0)
        assert!((0.0..1.0).contains(&hit.uv.0));
        assert!((0.0..1.0).contains(&hit.uv.1));
    }

    #[test]
    fn with_tile_size_rescales_uv() {
        let plane = Plane::ground(-1.0, Material::solid(Color::WHITE)).with_tile_size(4.0);
        assert!(approx(plane.tile_size, 4.0));
        let ray = Ray::new(Vec3::new(2.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = plane.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.uv.0, 0.5)); // (2.0/4.0).rem_euclid(1.0)
    }

    #[test]
    fn misses_when_parallel() {
        let plane = Plane::ground(-1.0, Material::solid(Color::new(0.5, 0.5, 0.5)));
        let ray = Ray::new(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
        assert!(plane.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn misses_when_plane_is_behind() {
        let plane = Plane::ground(-1.0, Material::solid(Color::new(0.5, 0.5, 0.5)));
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
        assert!(plane.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn hits_from_below_flips_shading_normal() {
        let plane = Plane::ground(-1.0, Material::solid(Color::new(0.5, 0.5, 0.5)));
        let ray = Ray::new(Vec3::new(0.0, -2.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let hit = plane.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.t, 1.0));
        assert!(approx(hit.normal.y, -1.0)); // faces the incoming ray
        assert!(!hit.front_face);
    }

    #[test]
    fn t_window_respected() {
        let plane = Plane::ground(-1.0, Material::solid(Color::new(0.5, 0.5, 0.5)));
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, -1.0, 0.0));
        assert!(plane.hit(&ray, 0.001, 0.5).is_none());
        assert!(plane.hit(&ray, 0.001, 2.0).is_some());
    }
}
