use std::{f64::consts, sync::Arc};

use crate::{
    assert_approx_eq,
    primitives::vector::{point, vector, Point, Transformation},
    shapes::Shape,
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
    let object = Arc::new(Shape::default());
    let result = light.lighting(object, &m, &position, &eye, &normal, false);
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
    let object = Arc::new(Shape::default());
    let result = light.lighting(object, &m, &position, &eye, &normal, false);
    assert_approx_eq!(result, Color::new_rgb(1.0, 1.0, 1.0));
}

#[test]
fn lighting_light_up() {
    let m = Material::default();
    let position = point(0., 0., 0.);
    let eye = vector(0., 0., -1.);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 10., -10.), Color::new_rgb(1., 1., 1.));
    let object = Arc::new(Shape::default());
    let result = light.lighting(object, &m, &position, &eye, &normal, false);
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
    let object = Arc::new(Shape::default());
    let result = light.lighting(object, &m, &position, &eye, &normal, false);
    assert_approx_eq!(result, Color::new_rgb(1.6364, 1.6364, 1.6364));
}

#[test]
fn lighting_light_behind() {
    let m = Material::default();
    let position = point(0., 0., 0.);
    let eye = vector(0., 0., -1.);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 0., 10.), Color::new_rgb(1., 1., 1.));
    let object = Arc::new(Shape::default());
    let result = light.lighting(object, &m, &position, &eye, &normal, false);
    assert_approx_eq!(result, Color::new_rgb(0.1, 0.1, 0.1));
}

#[test]
fn lighting_surface_in_shadow() {
    let m = Material::default();
    let position = point(0., 0., 0.);
    let eye = vector(0., 0., -1.);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 0., -10.), Color::new_rgb(1., 1., 1.));
    let object = Arc::new(Shape::default());
    let result = light.lighting(object, &m, &position, &eye, &normal, true);
    assert_approx_eq!(result, Color::new_rgb(0.1, 0.1, 0.1));
}

#[test]
fn stripe_constant_y() {
    let p = Pattern::new(STRIPE_X_WHITE_BLACK, Transformation::identity());
    let object = Arc::new(Shape::default());
    assert_approx_eq!(p.at(object.clone(), &point(0., 0., 0.)), Color::white());
    assert_approx_eq!(p.at(object.clone(), &point(0., 1., 0.)), Color::white());
    assert_approx_eq!(p.at(object.clone(), &point(0., 2., 0.)), Color::white());
}

#[test]
fn stripe_constant_z() {
    let p = Pattern::new(STRIPE_X_WHITE_BLACK, Transformation::identity());
    let object = Arc::new(Shape::default());
    assert_approx_eq!(p.at(object.clone(), &point(0., 0., 0.)), Color::white());
    assert_approx_eq!(p.at(object.clone(), &point(0., 0., 1.)), Color::white());
    assert_approx_eq!(p.at(object.clone(), &point(0., 0., 2.)), Color::white());
}

#[test]
fn stripe_constant_x() {
    let p = Pattern::new(STRIPE_X_WHITE_BLACK, Transformation::identity());
    let object = Arc::new(Shape::default());
    assert_approx_eq!(p.at(object.clone(), &point(0., 0., 0.)), Color::white());
    assert_approx_eq!(p.at(object.clone(), &point(0.9, 0., 0.)), Color::white());
    assert_approx_eq!(p.at(object.clone(), &point(1., 0., 0.)), Color::black());
    assert_approx_eq!(p.at(object.clone(), &point(-0.1, 0., 0.)), Color::black());
    assert_approx_eq!(p.at(object.clone(), &point(-1.0, 0., 0.)), Color::black());
    assert_approx_eq!(p.at(object.clone(), &point(-1.1, 0., 0.)), Color::white());
}

#[test]
fn lighting_with_pattern() {
    let p = Pattern::new(STRIPE_X_WHITE_BLACK, Transformation::identity());
    let mut m = Material::default();
    m.ambient = 1.0;
    m.diffuse = 0.0;
    m.specular = 0.0;
    m.pattern = Some(p);
    let eye = vector(0., 0., -1.);
    let normal = vector(0., 0., -1.);
    let light = PointLight::new(point(0., 0., -10.), Color::white());
    let object = Arc::new(Shape::default());
    let c1 = light.lighting(
        object.clone(),
        &m,
        &point(0.9, 0.0, 0.0),
        &eye,
        &normal,
        false,
    );
    let c2 = light.lighting(
        object.clone(),
        &m,
        &point(1.1, 0.0, 0.0),
        &eye,
        &normal,
        false,
    );
    assert_approx_eq!(c1, Color::white());
    assert_approx_eq!(c2, Color::black());
}

#[test]
fn stripes_with_object_transformation() {
    let mut object = Shape::default();
    object.modify_transform(|transform| transform.scale(2., 2., 2.));
    let pattern = Pattern::new(STRIPE_X_WHITE_BLACK, Transformation::identity());
    let c = pattern.at(Arc::new(object), &point(1.5, 0., 0.));
    assert_approx_eq!(c, Color::white());
}

#[test]
fn stripes_with_pattern_transformation() {
    let object = Shape::default();
    let mut pattern = Pattern::new(STRIPE_X_WHITE_BLACK, Transformation::identity());
    pattern.modify_transform(|transform| transform.scale(2., 2., 2.));
    let c = pattern.at(Arc::new(object), &point(1.5, 0., 0.));
    assert_approx_eq!(c, Color::white());
}

#[test]
fn stripes_with_pattern_and_object_transformation() {
    let mut object = Shape::default();
    object.modify_transform(|transform| transform.scale(2., 2., 2.));
    let mut pattern = Pattern::new(STRIPE_X_WHITE_BLACK, Transformation::identity());
    pattern.modify_transform(|transform| transform.translate(0.5, 0., 0.));
    let c = pattern.at(Arc::new(object), &point(2.5, 0., 0.));
    assert_approx_eq!(c, Color::white());
}

#[test]
fn gradient() {
    let object = Arc::new(Shape::default());
    let pattern = Pattern::new(GRADIENT_X_WHITE_BLACK, Transformation::identity());
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0., 0., 0.)),
        Color::white()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0.25, 0., 0.)),
        Color::new_rgb(0.75, 0.75, 0.75)
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0.5, 0., 0.)),
        Color::new_rgb(0.5, 0.5, 0.5)
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0.75, 0., 0.)),
        Color::new_rgb(0.25, 0.25, 0.25)
    );
}

#[test]
fn ring() {
    let object = Arc::new(Shape::default());
    let pattern = Pattern::new(RING_XZ_WHITE_BLACK, Transformation::identity());
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0., 0., 0.)),
        Color::white()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(1., 0., 0.)),
        Color::black()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0., 0., 1.)),
        Color::black()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0.708, 0., 0.708)),
        Color::black()
    );
}

#[test]
fn checkers() {
    let object = Arc::new(Shape::default());
    let pattern = Pattern::new(CHECKERS_WHITE_BLACK, Transformation::identity());
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0., 0., 0.)),
        Color::white()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0.99, 0., 0.)),
        Color::white()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(1.01, 0., 0.)),
        Color::black()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0., 0.99, 0.)),
        Color::white()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0., 1.01, 0.)),
        Color::black()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0., 0., 0.99)),
        Color::white()
    );
    assert_approx_eq!(
        pattern.at(Arc::clone(&object), &point(0., 0., 1.01)),
        Color::black()
    );
}
