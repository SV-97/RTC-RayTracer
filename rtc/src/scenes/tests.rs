use super::*;

use crate::{
    assert_approx_eq,
    primitives::{
        ray::Ray,
        vector::{point, vector},
    },
    shading::{Color, PointLight},
    shapes::{Intersection, Shape},
};

#[test]
fn intersect() {
    let w = World::default();
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let xs = w.intersect(&r);
    assert_eq!(xs.len(), 4);
    assert_approx_eq!(xs[0].t, 4.0);
    assert_approx_eq!(xs[1].t, 4.5);
    assert_approx_eq!(xs[2].t, 5.5);
    assert_approx_eq!(xs[3].t, 6.0);
}

#[test]
fn shade_intersection() {
    let w = World::default();
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let s = &w.objects[0];
    let i = Intersection::new(4., s);
    let comps = i.prepare_computations(&r);
    let c = w.shade_hit(&comps);
    assert_approx_eq!(c, Color::new_rgb(0.38066, 0.47583, 0.2855));
}

#[test]
fn shade_intersection_inside() {
    let mut w = World::default();
    w.lights = vec![PointLight::new(
        point(0., 0.25, 0.),
        Color::new_rgb(1., 1., 1.),
    )];
    let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
    let s = &w.objects[1];
    let i = Intersection::new(0.5, s);
    let comps = i.prepare_computations(&r);
    let c = w.shade_hit(&comps);
    assert_approx_eq!(c, Color::new_rgb(0.90498, 0.90498, 0.90498));
}

#[test]
fn color_at_miss() {
    let w = World::default();
    let r = Ray::new(point(0., 0., -5.), vector(0., 1., 0.));
    let c = w.color_at(&r);
    assert_approx_eq!(c, Color::new_rgb(0., 0., 0.));
}

#[test]
fn color_at_hit() {
    let w = World::default();
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let c = w.color_at(&r);
    assert_approx_eq!(c, Color::new_rgb(0.38066, 0.47583, 0.2855));
}

#[test]
fn color_at_from_inside() {
    let mut w = World::default();
    let outer = &mut w.objects[0];
    outer.material_mut().ambient = 1.0;
    drop(outer);
    let inner = &mut w.objects[1];
    inner.material_mut().ambient = 1.0;
    let inner = &w.objects[1];

    let r = Ray::new(point(0., 0., 0.75), vector(0., 0., -1.));
    let c = w.color_at(&r);
    assert_approx_eq!(c, inner.material().color);
}
