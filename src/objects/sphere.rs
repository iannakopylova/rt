//! Sphere primitive (center + radius).

use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

/// Sphere defined by center and radius, with a solid material.
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius: radius.abs(),
            material,
        }
    }

    pub fn with_albedo(center: Vec3, radius: f64, albedo: Color) -> Self {
        Self::new(center, radius, Material::solid(albedo))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Quadratic in the "half_b" form (avoids the classic 2·b factor).
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Nearest root in [t_min, t_max].
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward = (point - self.center) / self.radius;
        Some(HitRecord::from_outward_normal(
            root,
            point,
            outward,
            ray,
            self.material,
            sphere_uv(outward),
        ))
    }
}

/// Spherical UV from the **outward** unit normal (i.e. `(point - center) / radius`).
///
/// `u = 0.5 + atan2(z, x) / (2π)`, `v = 0.5 - asin(y) / π` (RT-018).
fn sphere_uv(outward: Vec3) -> (f64, f64) {
    use std::f64::consts::PI;
    let u = 0.5 + outward.z.atan2(outward.x) / (2.0 * PI);
    // Clamp guards fp drift pushing the argument just past asin's [-1, 1] domain.
    let v = 0.5 - outward.y.clamp(-1.0, 1.0).asin() / PI;
    (u, v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Camera;

    fn red_sphere() -> Sphere {
        Sphere::with_albedo(Vec3::new(0.0, 0.0, -5.0), 1.0, Color::new(1.0, 0.0, 0.0))
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn center_is_configurable() {
        let s = Sphere::with_albedo(Vec3::new(1.0, 1.0, 1.0), 0.5, Color::WHITE);
        assert_eq!(s.center, Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(s.radius, 0.5);
    }

    #[test]
    fn ray_hits_front_surface() {
        let sphere = red_sphere();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));
        let hit = sphere.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert!(approx(hit.t, 4.0));
        assert!(approx(hit.point.z, -4.0));
        assert!(approx(hit.normal.z, 1.0)); // outward (+Z) faces the camera
        assert!(hit.front_face);
        assert_eq!(hit.material.albedo, Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn uv_at_equator_front() {
        // Front hit (outward +Z): u = 0.5 + atan2(1, 0)/(2π) = 0.75, v = 0.5 - asin(0)/π = 0.5.
        let sphere = red_sphere();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));
        let hit = sphere.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.uv.0, 0.75));
        assert!(approx(hit.uv.1, 0.5));
    }

    #[test]
    fn uv_at_poles() {
        let sphere = red_sphere(); // center (0,0,-5), radius 1
        let top = Ray::new(Vec3::new(0.0, 3.0, -5.0), Vec3::new(0.0, -1.0, 0.0));
        let hit_top = sphere.hit(&top, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit_top.uv.1, 0.0)); // north pole (y = +radius) -> v = 0

        let bottom = Ray::new(Vec3::new(0.0, -3.0, -5.0), Vec3::new(0.0, 1.0, 0.0));
        let hit_bottom = sphere.hit(&bottom, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit_bottom.uv.1, 1.0)); // south pole (y = -radius) -> v = 1
    }

    #[test]
    fn uv_wraps_around_the_back() {
        let sphere = red_sphere();
        // Back hit (outward -Z): u = 0.5 + atan2(-1, 0)/(2π) = 0.25.
        let ray = Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(approx(hit.uv.0, 0.25));
        assert!((0.0..=1.0).contains(&hit.uv.0));
        assert!((0.0..=1.0).contains(&hit.uv.1));
    }

    #[test]
    fn ray_misses_beside_sphere() {
        let sphere = red_sphere();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
        assert!(sphere.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn from_inside_hits_far_side() {
        let sphere = red_sphere();
        // Start just inside the front surface, travel toward -Z.
        let ray = Ray::new(Vec3::new(0.0, 0.0, -4.1), Vec3::new(0.0, 0.0, -1.0));
        let hit = sphere.hit(&ray, 0.001, f64::INFINITY).unwrap();
        assert!(hit.t > 0.0);
        assert!(!hit.front_face); // ray is leaving from inside
        assert!(approx(hit.point.z, -6.0));
    }

    #[test]
    fn t_window_rejects_near_hit() {
        let sphere = red_sphere();
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));
        // Front hit is at t=4; require t > 4.5 so only the back root remains — still outside window.
        assert!(sphere.hit(&ray, 4.5, 4.9).is_none());
        let back = sphere.hit(&ray, 4.5, 10.0).unwrap();
        assert!(approx(back.t, 6.0));
    }

    #[test]
    fn camera_grid_sees_a_disc() {
        // Closer sphere so the silhouette fills enough of a low-res frame.
        let sphere = Sphere::with_albedo(Vec3::new(0.0, 0.0, -3.0), 1.0, Color::new(1.0, 0.0, 0.0));
        let cam = Camera::look_at(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        );

        let n = 32u32;
        let mut hits = 0usize;
        for y in 0..n {
            for x in 0..n {
                let ray = cam.ray_through_pixel(x, y, n, n);
                if sphere.hit(&ray, 0.001, f64::INFINITY).is_some() {
                    hits += 1;
                }
            }
        }

        assert!(hits > 80, "expected a filled disc, got {hits} hits");
        assert!(hits < n as usize * n as usize / 2, "disc should not fill the frame");

        let center = cam.get_ray(0.5, 0.5);
        assert!(sphere.hit(&center, 0.001, f64::INFINITY).is_some());

        let corner = cam.get_ray(0.0, 0.0);
        assert!(sphere.hit(&corner, 0.001, f64::INFINITY).is_none());
    }
}
