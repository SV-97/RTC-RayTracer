use super::Color;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new_rgb(1., 1., 1.),
            ambient: 1.,
            diffuse: 1.,
            specular: 1.,
            shininess: 1.,
        }
    }
}
