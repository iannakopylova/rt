//! Surface appearance for shading (extended in RT-008 / bonus tickets).

use crate::vec3::Color;

/// Diffuse albedo for now; reflection/refraction/textures plug in later.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub albedo: Color,
}

impl Material {
    pub fn solid(albedo: Color) -> Self {
        Self { albedo }
    }
}
