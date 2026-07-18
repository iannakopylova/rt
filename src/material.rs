//! Surface appearance for shading (RT-008 + RT-016 reflection).

use crate::vec3::Color;

/// Diffuse albedo with optional mirror-like reflectivity (0 = matte, 1 = mirror).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub albedo: Color,
    /// Fraction of the bounce that follows the mirror reflection ray when
    /// reflections are enabled in the tracer (`0.0` … `1.0`).
    pub reflectivity: f64,
}

impl Material {
    pub fn solid(albedo: Color) -> Self {
        Self {
            albedo,
            reflectivity: 0.0,
        }
    }

    /// Metal / mirror: `reflectivity` blends Lambertian with recursive reflection.
    pub fn metal(albedo: Color, reflectivity: f64) -> Self {
        Self {
            albedo,
            reflectivity: reflectivity.clamp(0.0, 1.0),
        }
    }
}
