use std::f64::consts;
use std::sync::Arc;

use super::*;

use crate::{
    assert_approx_eq,
    primitives::{
        ray::Ray,
        vector::{point, vector, Point, Transformation},
    },
    shading::Material,
};

#[test]
fn intersect_ray_sphere_2() {
    let s = Shape::default();
    let s = Arc::new(s);
    let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let is = (s.intersect)(Arc::clone(&s), &ray).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0], &Intersection::new(4.0, Arc::clone(&s)));
    assert_approx_eq!(is[1], &Intersection::new(6.0, s));
}

#[test]
fn intersect_ray_sphere_1() {
    let s = Shape::default();
    let s = Arc::new(s);
    let ray = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
    let is = (s.intersect)(Arc::clone(&s), &ray).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0], &Intersection::new(5.0, Arc::clone(&s)));
    assert_approx_eq!(is[1], &Intersection::new(5.0, s));
}

#[test]
fn intersect_ray_sphere_inside() {
    let s = Shape::default();
    let s = Arc::new(s);
    let ray = Ray::new(Point::origin(), vector(0., 0., 1.));
    let is = (s.intersect)(Arc::clone(&s), &ray).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0], &Intersection::new(-1.0, Arc::clone(&s)));
    assert_approx_eq!(is[1], &Intersection::new(1.0, s));
}

#[test]
fn intersect_ray_sphere_behind() {
    let s = Shape::default();
    let s = Arc::new(s);
    let ray = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
    let is = (s.intersect)(Arc::clone(&s), &ray).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0], &Intersection::new(-6.0, Arc::clone(&s)));
    assert_approx_eq!(is[1], &Intersection::new(-4.0, s));
}

#[test]
fn hit_all_positive() {
    let s = Shape::default();
    let s = Arc::new(s);
    let i1 = Intersection::new(1., Arc::clone(&s));
    let i2 = Intersection::new(2., s);
    let is = Intersections::new(vec![i2, i1.clone()]);
    assert_approx_eq!(is.hit().unwrap(), &i1);
}

#[test]
fn hit_some_negative() {
    let s = Shape::default();
    let s = Arc::new(s);
    let i1 = Intersection::new(-1., Arc::clone(&s));
    let i2 = Intersection::new(1., s);
    let is = Intersections::new(vec![i2.clone(), i1]);
    assert_approx_eq!(is.hit().unwrap(), &i2);
}

#[test]
fn hit_all_negative() {
    let s = Shape::default();
    let s = Arc::new(s);
    let i1 = Intersection::new(-2., Arc::clone(&s));
    let i2 = Intersection::new(-1., s);
    let is = Intersections::new(vec![i2, i1]);
    assert!(is.hit().is_none());
}

#[test]
fn hit_always_lowest() {
    let s = Shape::default();
    let s = Arc::new(s);
    let i1 = Intersection::new(5., Arc::clone(&s));
    let i2 = Intersection::new(7., Arc::clone(&s));
    let i3 = Intersection::new(-3., Arc::clone(&s));
    let i4 = Intersection::new(2., s);
    let is = Intersections::new(vec![i1, i2, i3, i4.clone()]);
    assert_approx_eq!(is.hit().unwrap(), &i4);
}

#[test]
fn sphere_transform_get() {
    let s = Shape::default();
    assert_approx_eq!(s.transform(), &Transformation::identity());
}

#[test]
fn sphere_transform_set() {
    let mut s = Shape::default();
    let t = Transformation::new_translation(2., 3., 4.);
    s.set_transform(t.clone());
    assert_approx_eq!(s.transform(), &t);
}

#[test]
fn interset_ray_scaled_sphere() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let mut s = Shape::default();
    let t = Transformation::new_scaling(2., 2., 2.);
    s.set_transform(t.clone());
    let s = Arc::new(s);
    let is = (s.intersect)(s, &r).unwrap();
    assert_eq!(is.len(), 2);
    assert_approx_eq!(is[0].t, 3.);
    assert_approx_eq!(is[1].t, 7.);
}

#[test]
fn interset_ray_translated_sphere() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let mut s = Shape::default();
    let t = Transformation::new_translation(5., 0., 0.);
    s.set_transform(t.clone());
    let s = Arc::new(s);
    let is = (s.intersect)(s, &r);
    assert!(is.is_none());
}

#[test]
fn normal_on_x_axis() {
    let s = Shape::default();
    let s = Arc::new(s);
    let n = (s.normal_at)(s, &point(1., 0., 0.));
    assert_approx_eq!(n, &vector(1., 0., 0.));
}

#[test]
fn normal_on_y_axis() {
    let s = Shape::default();
    let s = Arc::new(s);
    let n = (s.normal_at)(s, &point(0., 1., 0.));
    assert_approx_eq!(n, &vector(0., 1., 0.));
}

#[test]
fn normal_on_z_axis() {
    let s = Shape::default();
    let s = Arc::new(s);
    let n = (s.normal_at)(s, &point(0., 0., 1.));
    assert_approx_eq!(n, &vector(0., 0., 1.));
}

#[test]
fn normal_nonaxial() {
    let s = Shape::default();
    let s = Arc::new(s);
    let v = (3.0_f64).sqrt() / 3.;
    let n = (s.normal_at)(s, &point(v, v, v));
    assert_approx_eq!(n, &vector(v, v, v));
}

#[test]
fn normal_normalization() {
    let s = Shape::default();
    let s = Arc::new(s);
    let v = (3.0_f64).sqrt() / 3.;
    let n = (s.normal_at)(s, &point(v, v, v));
    assert_approx_eq!(n, &n.clone().unit());
}

#[test]
fn normal_of_translated_sphere() {
    let mut s = Shape::default();
    s.modify_transform(|t| t.translate(0., 1., 0.));
    let s = Arc::new(s);
    let n = (s.normal_at)(s, &point(0., 1.70711, -0.70711));
    assert_approx_eq!(n, &vector(0., 0.70711, -0.70711));
}

#[test]
fn normal_of_transformed_sphere() {
    let mut s = Shape::default();
    s.modify_transform(|t| t.rotate_z(consts::PI / 5.).scale(1., 0.5, 1.));
    let s = Arc::new(s);
    let a = consts::SQRT_2 / 2.0;
    let n = (s.normal_at)(s, &point(0., a, -a));
    assert_approx_eq!(n, &vector(0., 0.97014, -0.24254));
}

#[test]
fn precompute_intersection() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let shape = Shape::default();
    let shape = Arc::new(shape);
    let i = Intersection::new(4., Arc::clone(&shape));
    let comps = (i.clone()).prepare_computations(&r);
    assert_approx_eq!(comps.t, i.t);
    assert_approx_eq!(comps.object, &shape);
    assert_approx_eq!(comps.point, &point(0., 0., -1.));
    assert_approx_eq!(comps.eye, &vector(0., 0., -1.));
    assert_approx_eq!(comps.normal, &vector(0., 0., -1.));
}

#[test]
fn precompute_inside() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let shape = Shape::default();
    let shape = Arc::new(shape);
    let i = Intersection::new(4., shape);
    let comps = i.prepare_computations(&r);
    assert!(!comps.inside);

    let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
    let shape = Shape::default();
    let shape = Arc::new(shape);
    let i = Intersection::new(1., shape);
    let comps = i.prepare_computations(&r);
    assert_approx_eq!(comps.point, &point(0., 0., 1.));
    assert_approx_eq!(comps.eye, &vector(0., 0., -1.));
    assert!(comps.inside);
    assert_approx_eq!(comps.normal, &vector(0., 0., -1.));
}

#[test]
fn plane_normal() {
    let p = Shape::new_plane(Material::default(), Transformation::identity());
    let p = Arc::new(p);
    let n1 = (p.normal_at)(p.clone(), &point(0., 0., 0.));
    let n2 = (p.normal_at)(p.clone(), &point(10., 0., -10.));
    let n3 = (p.normal_at)(p.clone(), &point(-5., 0., 150.));
    assert_approx_eq!(n1, &vector(0., 1., 0.));
    assert_approx_eq!(n2, &vector(0., 1., 0.));
    assert_approx_eq!(n3, &vector(0., 1., 0.));
}

#[test]
fn plane_intersect_parallel() {
    let p = Shape::new_plane(Material::default(), Transformation::identity());
    let p = Arc::new(p);
    let r = Ray::new(point(0., 10., 0.), vector(0., 0., 1.));
    let xs = (p.intersect)(p, &r);
    assert!(xs.is_none());
}

#[test]
fn plane_intersect_coplanar() {
    let p = Shape::new_plane(Material::default(), Transformation::identity());
    let p = Arc::new(p);
    let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
    let xs = (p.intersect)(p, &r);
    assert!(xs.is_none());
}

#[test]
fn plane_intersect_from_above() {
    let p = Shape::new_plane(Material::default(), Transformation::identity());
    let p = Arc::new(p);
    let r = Ray::new(point(0., 1., 0.), vector(0., -1., 0.));
    let xs = (p.intersect)(p.clone(), &r).unwrap();
    assert_eq!(xs.len(), 1);
    assert_approx_eq!(xs[0].t, 1.);
    assert_approx_eq!(xs[0].object, &p);
}

#[test]
fn plane_intersect_from_below() {
    let p = Shape::new_plane(Material::default(), Transformation::identity());
    let p = Arc::new(p);
    let r = Ray::new(point(0., -1., 0.), vector(0., 1., 0.));
    let xs = (p.intersect)(p.clone(), &r).unwrap();
    assert_eq!(xs.len(), 1);
    assert_approx_eq!(xs[0].t, 1.);
    assert_approx_eq!(xs[0].object, &p);
}
