use super::Color;
use crate::primitives::vector::Point;

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
}
