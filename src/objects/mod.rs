//! Shared surface-hit API for primitives.
//!
//! Stubbed here so object tickets (RT-004+) can land before the full tracer (RT-009).
//! Signature matches the RT-009 contract: `hit(ray, t_min, t_max) -> Option<HitRecord>`.

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

pub use cube::Cube;
pub use cylinder::Cylinder;
pub use plane::Plane;
pub use sphere::Sphere;

/// Record of the nearest surface intersection along a ray.
#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    /// Unit normal oriented against the incoming ray (`front_face == true` ⇒ outward).
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
    /// Object-specific surface UV in `[0, 1]²`, used by [`crate::texture::sample_albedo`]
    /// (RT-018). Meaningless for materials without a texture.
    pub uv: (f64, f64),
}

impl HitRecord {
    /// Build a hit from an **outward** geometric normal; flips it if the ray is inside the surface.
    pub fn from_outward_normal(
        t: f64,
        point: Vec3,
        outward_normal: Vec3,
        ray: &Ray,
        material: Material,
        uv: (f64, f64),
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            point,
            normal,
            front_face,
            material,
            uv,
        }
    }
}

/// Anything that can be intersected by a primary or shadow ray.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
