use std::f64::consts;

use crate::{
    assert_approx_eq,
    primitives::vector::{point, vector, Point},
};

use super::*;

#[test]
fn new() {
    let i = Color::new_rgb(1., 1., 1.);
    let p = Point::origin();
    let light = PointLight::new(p.clone(), i.clone());
    assert_approx_eq!(light.intensity, i);
    assert_approx_eq!(light.position, &p);
}

#[test]
fn lighting_eye_between() {
    let m = Material::default();
    let position = point(0., 0., 0.);
    let eye = vector(0., 0., -1.);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 0., -10.), Color::new_rgb(1., 1., 1.));
    let result = light.lighting(&m, &position, &eye, &normal);
    assert_approx_eq!(result, Color::new_rgb(1.9, 1.9, 1.9));
}

#[test]
fn lighting_eye_up() {
    let m = Material::default();
    let position = point(0., 0., 0.);
    let a = consts::SQRT_2 / 2.0;
    let eye = vector(0., a, -a);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 0., -10.), Color::new_rgb(1., 1., 1.));
    let result = light.lighting(&m, &position, &eye, &normal);
    assert_approx_eq!(result, Color::new_rgb(1.0, 1.0, 1.0));
}

#[test]
fn lighting_light_up() {
    let m = Material::default();
    let position = point(0., 0., 0.);
    let a = consts::SQRT_2 / 2.0;
    let eye = vector(0., 0., -1.);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 10., -10.), Color::new_rgb(1., 1., 1.));
    let result = light.lighting(&m, &position, &eye, &normal);
    assert_approx_eq!(result, Color::new_rgb(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_light_up_eye_down() {
    let m = Material::default();
    let position = point(0., 0., 0.);
    let a = consts::SQRT_2 / 2.0;
    let eye = vector(0., -a, -a);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 10., -10.), Color::new_rgb(1., 1., 1.));
    let result = light.lighting(&m, &position, &eye, &normal);
    assert_approx_eq!(result, Color::new_rgb(1.6364, 1.6364, 1.6364));
}

#[test]
fn lighting_light_behind() {
    let m = Material::default();
    let position = point(0., 0., 0.);
    let eye = vector(0., 0., -1.);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 0., 10.), Color::new_rgb(1., 1., 1.));
    let result = light.lighting(&m, &position, &eye, &normal);
    assert_approx_eq!(result, Color::new_rgb(0.1, 0.1, 0.1));
}
