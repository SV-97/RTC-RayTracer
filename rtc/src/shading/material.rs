use crate::primitives::approx_eq::ApproxEq;

use super::{Color, Pattern};

#[derive(Debug, Clone)]

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Material {
    pub color: Color,
    pub pattern: Option<Pattern>,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new_with_pattern(
        color: Color,
        pattern: Option<Pattern>,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            pattern,
        }
    }
    pub fn new(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Self {
        Self::new_with_pattern(
            color,
            None,
            ambient,
            diffuse,
            specular,
            shininess,
        )
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(Color::new_rgb(1., 1., 1.), 0.1, 0.9, 0.9, 200.)
    }
}

impl ApproxEq for &Material {
    fn approx_eq(self, other: Self) -> bool {
        self.color.approx_eq(other.color)
            && self.ambient.approx_eq(other.ambient)
            && self.diffuse.approx_eq(other.diffuse)
            && self.specular.approx_eq(other.specular)
            && self.shininess.approx_eq(other.shininess)
    }
}
