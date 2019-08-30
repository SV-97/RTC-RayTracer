use crate::primitives::vector::{Point, ScalarProd, Vec3D};

use super::{Color, Material};

#[derive(Debug, Clone, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        PointLight {
            intensity,
            position,
        }
    }

    pub fn lighting(
        &self,
        material: &Material,
        point: &Point,
        eye: &Vec3D,
        normal: &Vec3D,
    ) -> Color {
        let effective_color = material.color * self.intensity;
        let light_v = (&self.position - point).unit();
        let ambient = effective_color * material.ambient;
        let light_dot_normal: f32 = (&light_v).scalar_prod(normal) as f32;
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            let diffuse = Color::black();
            let specular = Color::black();
            (diffuse, specular)
        } else {
            let diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflect_v = (-light_v).reflect(normal);
            let reflect_dot_eye = reflect_v.scalar_prod(eye);
            let specular = if reflect_dot_eye <= 0.0 {
                Color::black()
            } else {
                let factor: f32 = (reflect_dot_eye as f32).powf(material.shininess);
                self.intensity * material.specular * factor
            };
            (diffuse, specular)
        };
        ambient + diffuse + specular
    }
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight::new(Point::origin(), Color::default())
    }
}