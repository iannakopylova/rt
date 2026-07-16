//! Finite Y-aligned cylinder (side wall + disk caps).
//!
//! # Shape
//! Axis is world **+Y**. The bottom cap is centered at [`Cylinder::base`]; the top cap
//! sits at `base.y + height`. Not an infinite tube — height is always finite.
//!
//! ```ignore
//! // Midpoint style (Scenes 3–4):
//! let cyl = Cylinder::from_midpoint(
//!     Vec3::new(-1.5, 0.0, -3.0),
//!     0.5,
//!     2.0,
//!     Material::solid(Color::new(0.2, 0.8, 0.3)),
//! );
//! ```

use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

const EPS: f64 = 1e-12;

/// Finite cylinder standing on +Y.
#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    /// Center of the bottom disk.
    pub base: Vec3,
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

impl Cylinder {
    pub fn new(base: Vec3, radius: f64, height: f64, material: Material) -> Self {
        Self {
            base,
            radius: radius.abs(),
            height: height.abs(),
            material,
        }
    }

    /// Axis midpoint + full height (same framing many scene graphs use).
    pub fn from_midpoint(mid: Vec3, radius: f64, height: f64, material: Material) -> Self {
        let h = height.abs();
        Self::new(
            Vec3::new(mid.x, mid.y - h * 0.5, mid.z),
            radius,
            h,
            material,
        )
    }

    pub fn with_albedo(mid: Vec3, radius: f64, height: f64, albedo: Color) -> Self {
        Self::from_midpoint(mid, radius, height, Material::solid(albedo))
    }

    pub fn y_min(self) -> f64 {
        self.base.y
    }

    pub fn y_max(self) -> f64 {
        self.base.y + self.height
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut best_t = t_max;
        let mut best_outward: Option<Vec3> = None;

        let mut consider = |t: f64, outward: Vec3| {
            if t > t_min && t < best_t {
                best_t = t;
                best_outward = Some(outward);
            }
        };

        self.hit_side(ray, &mut consider);
        self.hit_cap(ray, self.y_max(), Vec3::new(0.0, 1.0, 0.0), &mut consider);
        self.hit_cap(ray, self.y_min(), Vec3::new(0.0, -1.0, 0.0), &mut consider);

        let outward = best_outward?;
        let point = ray.at(best_t);
        Some(HitRecord::from_outward_normal(
            best_t,
            point,
            outward,
            ray,
            self.material,
        ))
    }
}

impl Cylinder {
    /// Infinite-tube quadratic in XZ, clipped to `[y_min, y_max]`.
    fn hit_side(&self, ray: &Ray, consider: &mut dyn FnMut(f64, Vec3)) {
        let ox = ray.origin.x - self.base.x;
        let oz = ray.origin.z - self.base.z;
        let dx = ray.direction.x;
        let dz = ray.direction.z;

        // half_b form on the XZ circle
        let a = dx * dx + dz * dz;
        if a < EPS {
            return; // ray parallel to axis — only caps can hit
        }

        let half_b = ox * dx + oz * dz;
        let c = ox * ox + oz * oz - self.radius * self.radius;
        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return;
        }

        let sqrt_d = disc.sqrt();
        for signed in [-1.0, 1.0] {
            let t = (-half_b + signed * sqrt_d) / a;
            let y = ray.origin.y + t * ray.direction.y;
            if y < self.y_min() || y > self.y_max() {
                continue;
            }
            let px = ray.origin.x + t * dx;
            let pz = ray.origin.z + t * dz;
            let mut outward = Vec3::new(px - self.base.x, 0.0, pz - self.base.z);
            if outward.length_squared() < EPS {
                outward = Vec3::new(1.0, 0.0, 0.0);
            } else {
                outward = outward.normalize();
            }
            consider(t, outward);
        }
    }

    fn hit_cap(&self, ray: &Ray, y_plane: f64, outward: Vec3, consider: &mut dyn FnMut(f64, Vec3)) {
        if ray.direction.y.abs() < EPS {
            return;
        }
        let t = (y_plane - ray.origin.y) / ray.direction.y;
        let p = ray.at(t);
        let dx = p.x - self.base.x;
        let dz = p.z - self.base.z;
        if dx * dx + dz * dz <= self.radius * self.radius {
            consider(t, outward);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn unit_cyl() -> Cylinder {
        // Mid (0,0,-5), r=1, h=2 → y in [-1, 1], front wall at z=-4
        Cylinder::with_albedo(Vec3::new(0.0, 0.0, -5.0), 1.0, 2.0, Color::new(0.1, 0.8, 0.2))
    }

    #[test]
    fn base_and_midpoint_agree() {
        let mid = Cylinder::from_midpoint(
            Vec3::new(1.0, 2.0, 3.0),
            0.5,
            4.0,
            Material::solid(Color::WHITE),
        );
        assert!(approx(mid.base.y, 0.0));
        assert!(approx(mid.y_max(), 4.0));
        assert!(approx(mid.base.x, 1.0));
    }

    #[test]
    fn hits_side_wall() {
        let cyl = unit_cyl();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));
        let hit = cyl.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.t, 4.0));
        assert!(approx(hit.point.z, -4.0));
        assert!(approx(hit.normal.z, 1.0));
        assert!(hit.front_face);
    }

    #[test]
    fn hits_top_cap() {
        // Mid at y=-2 → top at y=-1
        let cyl = Cylinder::with_albedo(Vec3::new(0.0, -2.0, -5.0), 1.0, 2.0, Color::WHITE);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = cyl.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.t, 1.0));
        assert!(approx(hit.point.y, -1.0));
        assert!(approx(hit.normal.y, 1.0));
    }

    #[test]
    fn hits_bottom_cap() {
        // Mid at y=2 → bottom at y=1
        let cyl = Cylinder::with_albedo(Vec3::new(0.0, 2.0, -5.0), 1.0, 2.0, Color::WHITE);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 1.0, 0.0));
        let hit = cyl.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.t, 1.0));
        assert!(approx(hit.point.y, 1.0));
        assert!(approx(hit.normal.y, -1.0));
    }

    #[test]
    fn misses_beside() {
        let cyl = unit_cyl();
        let ray = Ray::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(cyl.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn misses_when_behind() {
        let cyl = unit_cyl();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        assert!(cyl.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn misses_above_height_band() {
        let cyl = unit_cyl();
        let ray = Ray::new(Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(cyl.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn t_window_respected() {
        let cyl = unit_cyl();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));
        assert!(cyl.hit(&ray, 0.001, 3.0).is_none());
        assert!(cyl.hit(&ray, 0.001, 5.0).is_some());
    }
}
