use crate::{assert_approx_eq, primitives::vector::Point};

use super::*;

#[test]
fn new() {
    let i = Color::new_rgb(1., 1., 1.);
    let p = Point::origin();
    let light = PointLight::new(p.clone(), i.clone());
    assert_approx_eq!(light.intensity, i);
    assert_approx_eq!(light.position, &p);
}
