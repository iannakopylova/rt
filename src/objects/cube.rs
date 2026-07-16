//! Axis-aligned cube (AABB) via the slab method.
//!
//! Configure with [`Cube::from_corners`] (min/max) or [`Cube::from_center_extent`]
//! (center + full edge length). Face normals come from the slab that produced the hit.

use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

const PARALLEL_EPS: f64 = 1e-12;

/// Axis-aligned box stored as world-space corners.
#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    /// Build from arbitrary opposite corners (order does not matter).
    pub fn from_corners(a: Vec3, b: Vec3, material: Material) -> Self {
        Self {
            min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
            material,
        }
    }

    /// Center plus **full** edge length (not half-extent).
    pub fn from_center_extent(center: Vec3, edge: f64, material: Material) -> Self {
        let h = edge.abs() * 0.5;
        let extent = Vec3::new(h, h, h);
        Self::from_corners(center - extent, center + extent, material)
    }

    pub fn with_albedo(center: Vec3, edge: f64, albedo: Color) -> Self {
        Self::from_center_extent(center, edge, Material::solid(albedo))
    }

    pub fn center(self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_enter = t_min;
        let mut t_exit = t_max;
        let mut n_enter = Vec3::ZERO;
        let mut n_exit = Vec3::ZERO;

        let axes = [
            (ray.origin.x, ray.direction.x, self.min.x, self.max.x, Vec3::new(1.0, 0.0, 0.0)),
            (ray.origin.y, ray.direction.y, self.min.y, self.max.y, Vec3::new(0.0, 1.0, 0.0)),
            (ray.origin.z, ray.direction.z, self.min.z, self.max.z, Vec3::new(0.0, 0.0, 1.0)),
        ];

        for (origin, dir, bmin, bmax, axis) in axes {
            if !slab_axis(
                origin,
                dir,
                bmin,
                bmax,
                axis,
                &mut t_enter,
                &mut t_exit,
                &mut n_enter,
                &mut n_exit,
            ) {
                return None;
            }
            if t_enter > t_exit {
                return None;
            }
        }

        // Outside → entry face; inside → exit face.
        let (t, outward) = if t_enter > t_min {
            (t_enter, n_enter)
        } else if t_exit < t_max {
            (t_exit, n_exit)
        } else {
            return None;
        };

        if t <= t_min || t >= t_max {
            return None;
        }

        let point = ray.at(t);
        Some(HitRecord::from_outward_normal(
            t,
            point,
            outward,
            ray,
            self.material,
        ))
    }
}

/// Intersect one axis slab; updates enter/exit times and their outward normals.
fn slab_axis(
    origin: f64,
    dir: f64,
    bmin: f64,
    bmax: f64,
    axis: Vec3,
    t_enter: &mut f64,
    t_exit: &mut f64,
    n_enter: &mut Vec3,
    n_exit: &mut Vec3,
) -> bool {
    if dir.abs() < PARALLEL_EPS {
        return origin >= bmin && origin <= bmax;
    }

    let inv = 1.0 / dir;
    let mut t_near = (bmin - origin) * inv;
    let mut t_far = (bmax - origin) * inv;
    // Outward normals: min face → −axis, max face → +axis.
    let mut n_near = -axis;
    let mut n_far = axis;
    if t_near > t_far {
        std::mem::swap(&mut t_near, &mut t_far);
        std::mem::swap(&mut n_near, &mut n_far);
    }

    if t_near > *t_enter {
        *t_enter = t_near;
        *n_enter = n_near;
    }
    if t_far < *t_exit {
        *t_exit = t_far;
        *n_exit = n_far;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn unit_cube_at_z5() -> Cube {
        // Center (0,0,-5), edge 2 → faces at z=-4 and z=-6
        Cube::with_albedo(Vec3::new(0.0, 0.0, -5.0), 2.0, Color::new(0.1, 0.4, 1.0))
    }

    #[test]
    fn corners_and_center_constructors_match() {
        let a = Cube::from_corners(
            Vec3::new(-1.0, -1.0, -6.0),
            Vec3::new(1.0, 1.0, -4.0),
            Material::solid(Color::WHITE),
        );
        let b = Cube::from_center_extent(
            Vec3::new(0.0, 0.0, -5.0),
            2.0,
            Material::solid(Color::WHITE),
        );
        assert_eq!(a.min, b.min);
        assert_eq!(a.max, b.max);
        assert!(approx(a.center().z, -5.0));
    }

    #[test]
    fn hits_front_face() {
        let cube = unit_cube_at_z5();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));
        let hit = cube.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.t, 4.0));
        assert!(approx(hit.point.z, -4.0));
        assert!(approx(hit.normal.z, 1.0));
        assert!(hit.front_face);
    }

    #[test]
    fn misses_above() {
        let cube = unit_cube_at_z5();
        let ray = Ray::new(Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(cube.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn from_inside_hits_exit_face() {
        let cube = unit_cube_at_z5();
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = cube.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.t, 1.0));
        assert!(approx(hit.point.z, -6.0));
        // Outward on back face is −Z; ray travels −Z so shading normal flips to +Z.
        assert!(approx(hit.normal.z, 1.0));
        assert!(!hit.front_face);
    }

    #[test]
    fn misses_when_behind() {
        let cube = unit_cube_at_z5();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        assert!(cube.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn side_face_normal() {
        let cube = unit_cube_at_z5();
        let ray = Ray::new(Vec3::new(2.0, 0.0, -5.0), Vec3::new(-1.0, 0.0, 0.0));
        let hit = cube.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.point.x, 1.0));
        assert!(approx(hit.normal.x, 1.0));
    }

    #[test]
    fn t_window_respected() {
        let cube = unit_cube_at_z5();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));
        assert!(cube.hit(&ray, 0.001, 3.0).is_none());
        assert!(cube.hit(&ray, 0.001, 5.0).is_some());
    }
}
