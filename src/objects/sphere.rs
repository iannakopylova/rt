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
        ))
    }
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
