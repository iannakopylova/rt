//! Pinhole camera for primary rays.
//!
//! # Coordinate system
//! Right-handed. World **+Y is up**. The camera looks from `eye` toward `look_at`.
//!
//! # Normalized image plane
//! `get_ray(u, v)` uses **u, v ∈ [0, 1]**:
//! - `u = 0` left, `u = 1` right
//! - `v = 0` bottom, `v = 1` top
//!
//! Pixel centers map with:
//! `u = (x + 0.5) / width`, `v = 1.0 - (y + 0.5) / height`
//! (image `y = 0` is the top row, matching PPM top-to-bottom scan order).
//!
//! # Scene 4 — move the camera (same objects, new view)
//! ```ignore
//! // Front view (Scene 3 style)
//! let front = Camera::look_at(
//!     Vec3::new(0.0, 1.0, 4.0),   // eye
//!     Vec3::new(0.0, 0.0, 0.0),   // look-at
//!     Vec3::new(0.0, 1.0, 0.0),   // world up
//!     60.0,                      // vertical FOV (degrees)
//!     800.0 / 600.0,             // aspect
//! );
//!
//! // Alternate angle (Scene 4): same look-at, different eye
//! let side = Camera::look_at(
//!     Vec3::new(3.5, 2.0, 2.5),
//!     Vec3::new(0.0, 0.0, 0.0),
//!     Vec3::new(0.0, 1.0, 0.0),
//!     60.0,
//!     800.0 / 600.0,
//! );
//! ```

use crate::ray::Ray;
use crate::vec3::Vec3;

/// Configurable pinhole camera. Build with [`Camera::look_at`], then sample with [`Camera::get_ray`].
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    eye: Vec3,
    /// Orthonormal camera basis: `forward` points toward the scene.
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    /// `tan(vfov/2)` at unit focal length — scales the vertical half-extent of the image plane.
    half_height: f64,
    /// `half_height * aspect_ratio`
    half_width: f64,
}

impl Camera {
    /// Build a camera from eye position, look-at target, world-up, vertical FOV (degrees), and aspect ratio (`width/height`).
    pub fn look_at(
        eye: Vec3,
        look_at: Vec3,
        world_up: Vec3,
        vfov_degrees: f64,
        aspect_ratio: f64,
    ) -> Self {
        let vfov = vfov_degrees.clamp(1.0, 170.0);
        let aspect = aspect_ratio.max(1e-6);

        let forward = (look_at - eye).normalize();
        let (right, up) = orthonormal_frame(forward, world_up);

        let half_height = (vfov.to_radians() * 0.5).tan();
        let half_width = half_height * aspect;

        Self {
            eye,
            forward,
            right,
            up,
            half_height,
            half_width,
        }
    }

    pub fn eye(self) -> Vec3 {
        self.eye
    }

    pub fn forward(self) -> Vec3 {
        self.forward
    }

    /// Primary ray through normalized image coordinates `(u, v)` in `[0, 1]²`.
    ///
    /// `(0.5, 0.5)` is the image center and aligns with `forward` for a centered look-at.
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        // Map [0,1] → [-1,1] on the unit focal plane in front of the eye.
        let ndc_x = 2.0 * u - 1.0;
        let ndc_y = 2.0 * v - 1.0;
        let direction =
            self.forward + self.right * (ndc_x * self.half_width) + self.up * (ndc_y * self.half_height);
        Ray::new(self.eye, direction)
    }

    /// Convenience: ray through the center of pixel `(x, y)` for an image of size `width × height`.
    ///
    /// `y = 0` is the top row (PPM order).
    pub fn ray_through_pixel(self, x: u32, y: u32, width: u32, height: u32) -> Ray {
        let u = (x as f64 + 0.5) / width as f64;
        let v = 1.0 - (y as f64 + 0.5) / height as f64;
        self.get_ray(u, v)
    }
}

/// Build a right-handed orthonormal frame from view direction and world up.
fn orthonormal_frame(forward: Vec3, world_up: Vec3) -> (Vec3, Vec3) {
    let mut right = forward.cross(world_up);
    if right.length_squared() < 1e-12 {
        // Looking nearly parallel to world_up — pick a different reference axis.
        let fallback = if forward.x.abs() < 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };
        right = forward.cross(fallback);
    }
    let right = right.normalize();
    // Re-orthogonalize up so (right, up, forward) is orthonormal and right-handed.
    let up = right.cross(forward).normalize();
    (right, up)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn front_camera() -> Camera {
        Camera::look_at(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        )
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn center_ray_follows_look_at() {
        let cam = front_camera();
        let ray = cam.get_ray(0.5, 0.5);
        assert!(approx(ray.origin.x, 0.0));
        assert!(approx(ray.origin.y, 0.0));
        assert!(approx(ray.origin.z, 0.0));
        assert!(approx(ray.direction.x, 0.0));
        assert!(approx(ray.direction.y, 0.0));
        assert!(approx(ray.direction.z, -1.0));
    }

    #[test]
    fn corners_spread_outward() {
        let cam = front_camera();
        let top_left = cam.get_ray(0.0, 1.0).direction;
        assert!(top_left.x < 0.0);
        assert!(top_left.y > 0.0);
        assert!(top_left.z < 0.0);
    }

    #[test]
    fn moving_eye_changes_rays() {
        let a = Camera::look_at(
            Vec3::new(0.0, 1.0, 4.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            4.0 / 3.0,
        );
        let b = Camera::look_at(
            Vec3::new(3.5, 2.0, 2.5),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            4.0 / 3.0,
        );

        let ra = a.get_ray(0.5, 0.5);
        let rb = b.get_ray(0.5, 0.5);
        assert_ne!(ra.origin, rb.origin);
        assert_ne!(ra.direction, rb.direction);
    }

    #[test]
    fn pixel_helper_matches_normalized_uv() {
        let cam = front_camera();
        let via_pixel = cam.ray_through_pixel(50, 50, 101, 101);
        let u = (50.0 + 0.5) / 101.0;
        let v = 1.0 - (50.0 + 0.5) / 101.0;
        let via_uv = cam.get_ray(u, v);
        assert_eq!(via_pixel.origin, via_uv.origin);
        assert_eq!(via_pixel.direction, via_uv.direction);
    }

    #[test]
    fn parallel_up_still_builds_frame() {
        let cam = Camera::look_at(
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
        );
        let ray = cam.get_ray(0.5, 0.5);
        assert!((ray.direction.length() - 1.0).abs() < 1e-10);
        assert!(approx(ray.direction.y, 1.0));
    }

    #[test]
    fn forward_matches_center_ray_direction() {
        let cam = front_camera();
        let f = cam.forward();
        assert!((f.length() - 1.0).abs() < 1e-10);
        assert!(approx(f.x, 0.0));
        assert!(approx(f.y, 0.0));
        assert!(approx(f.z, -1.0));
        assert_eq!(cam.get_ray(0.5, 0.5).direction, f);
    }
}
