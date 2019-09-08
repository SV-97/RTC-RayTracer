use super::{Color, Material};

impl Material {
    pub fn glass() -> Self {
        Material::new(Color::new_rgb(1., 1., 1.), 0.1, 0.9, 0.9, 200., 0., 1., 1.5)
    }
}
