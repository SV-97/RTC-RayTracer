use super::Color;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(Color::new_rgb(1., 1., 1.), 1., 1., 1., 1.)
    }
}
