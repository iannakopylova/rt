use crate::vec3::Vec3;

/// A ray defined by an origin and a normalized direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_is_normalized() {
        let ray = Ray::new(Vec3::ZERO, Vec3::new(3.0, 0.0, 4.0));
        assert!((ray.direction.length() - 1.0).abs() < 1e-10);
        assert_eq!(ray.direction, Vec3::new(0.6, 0.0, 0.8));
    }

    #[test]
    fn point_along_ray() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.at(5.0), Vec3::new(5.0, 0.0, 0.0));
    }
}
