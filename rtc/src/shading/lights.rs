use crate::primitives::vector::{point, Point};
use super::Color;

pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {
    pub fn new(point: Point, intensity: Color) {
        PointLight {
            point,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn new() {
        let i = Color::new_rgb(1., 1., 1.);
        let p = Point::origin();
        let light = PointLight::new(p.clone(), i.clone());
        assert_approx_eq!(light.intensity, &i);
        assert_approx_eq!(light.position, &p);
    }
}