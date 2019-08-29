use crate::{
    assert_approx_eq,
    primitives::{
        ray::Ray,
        vector::{point, vector, Point, Transformation},
    },
};

use super::*;

#[test]
fn intersect_ray_sphere_2() {
    let s = Sphere::default();
    let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let is = s.intersect(&ray).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0], &Intersection::new(4.0, &s));
    assert_approx_eq!(is[1], &Intersection::new(6.0, &s));
}

#[test]
fn intersect_ray_sphere_1() {
    let s = Sphere::default();
    let ray = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
    let is = s.intersect(&ray).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0], &Intersection::new(5.0, &s));
    assert_approx_eq!(is[1], &Intersection::new(5.0, &s));
}

#[test]
fn intersect_ray_sphere_inside() {
    let s = Sphere::default();
    let ray = Ray::new(Point::origin(), vector(0., 0., 1.));
    let is = s.intersect(&ray).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0], &Intersection::new(-1.0, &s));
    assert_approx_eq!(is[1], &Intersection::new(1.0, &s));
}

#[test]
fn intersect_ray_sphere_behind() {
    let s = Sphere::default();
    let ray = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
    let is = s.intersect(&ray).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0], &Intersection::new(-6.0, &s));
    assert_approx_eq!(is[1], &Intersection::new(-4.0, &s));
}

#[test]
fn hit_all_positive() {
    let s = Sphere::default();
    let i1 = Intersection::new(1., &s);
    let i2 = Intersection::new(2., &s);
    let is = Intersections::new(vec![i2, i1.clone()]);
    assert_approx_eq!(is.hit().unwrap(), &i1);
}

#[test]
fn hit_some_negative() {
    let s = Sphere::default();
    let i1 = Intersection::new(-1., &s);
    let i2 = Intersection::new(1., &s);
    let is = Intersections::new(vec![i2.clone(), i1]);
    assert_approx_eq!(is.hit().unwrap(), &i2);
}

#[test]
fn hit_all_negative() {
    let s = Sphere::default();
    let i1 = Intersection::new(-2., &s);
    let i2 = Intersection::new(-1., &s);
    let is = Intersections::new(vec![i2, i1]);
    assert_eq!(is.hit(), None);
}

#[test]
fn hit_always_lowest() {
    let s = Sphere::default();
    let i1 = Intersection::new(5., &s);
    let i2 = Intersection::new(7., &s);
    let i3 = Intersection::new(-3., &s);
    let i4 = Intersection::new(2., &s);
    let is = Intersections::new(vec![i1, i2, i3, i4.clone()]);
    assert_approx_eq!(is.hit().unwrap(), &i4);
}

#[test]
fn sphere_transform_get() {
    let s = Sphere::default();
    assert_approx_eq!(s.get_transform(), &Transformation::identity());
}

#[test]
fn sphere_transform_set() {
    let mut s = Sphere::default();
    let t = Transformation::new_translation(2., 3., 4.);
    s.set_transform(t.clone());
    assert_approx_eq!(s.get_transform(), &t);
}

#[test]
fn interset_ray_scaled_sphere() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let mut s = Sphere::default();
    let t = Transformation::new_scaling(2., 2., 2.);
    s.set_transform(t.clone());
    let is = s.intersect(&r).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0].t, 3.);
    assert_approx_eq!(is[1].t, 7.);
}

#[test]
fn interset_ray_translated_sphere() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let mut s = Sphere::default();
    let t = Transformation::new_translation(5., 0., 0.);
    s.set_transform(t.clone());
    let is = s.intersect(&r);
    assert!(is.is_none());
}

#[test]
fn normal_on_x_axis() {
    let s = Sphere::default();
    let n = s.normal_at(&point(1., 0., 0.));
    assert_approx_eq!(n, &vector(1., 0., 0.));
}

#[test]
fn normal_on_y_axis() {
    let s = Sphere::default();
    let n = s.normal_at(&point(0., 1., 0.));
    assert_approx_eq!(n, &vector(0., 1., 0.));
}

#[test]
fn normal_on_z_axis() {
    let s = Sphere::default();
    let n = s.normal_at(&point(0., 0., 1.));
    assert_approx_eq!(n, &vector(0., 0., 1.));
}

#[test]
fn normal_nonaxial() {
    let s = Sphere::default();
    let v = (3.0_f64).sqrt() / 3.;
    let n = s.normal_at(&point(v, v, v));
    assert_approx_eq!(n, &vector(v, v, v));
}

#[test]
fn normal_normalization() {
    let s = Sphere::default();
    let v = (3.0_f64).sqrt() / 3.;
    let n = s.normal_at(&point(v, v, v));
    assert_approx_eq!(n, &n.clone().unit());
}

#[test]
fn normal_of_translated_sphere() {
    let mut s = Sphere::default();
    s.get_transform_mut().translate(0., 1., 0.);
    let n = s.normal_at(&point(0., 1.70711, -0.70711));
    assert_approx_eq!(n, &vector(0., 0.70711, -0.70711));
}

#[test]
fn normal_of_transformed_sphere() {
    use std::f64::consts;
    let mut s = Sphere::default();
    s.get_transform_mut()
        .rotate_z(consts::PI / 5.)
        .scale(1., 0.5, 1.);
    let a = consts::SQRT_2 / 2.0;
    let n = s.normal_at(&point(0., a, -a));
    assert_approx_eq!(n, &vector(0., 0.97014, -0.24254));
}