use std::sync::Arc;

use super::prelude::*;

use crate::{
    primitives::{
        ray::Ray,
        vector::{vector, Point, Transformation, Vec3D},
    },
    shading::Material,
};

pub static CUBE: ShapeFuncs = (intersect, normal_at);

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1. - origin;
    let tmax_numerator = 1. - origin;

    let tmin = tmin_numerator / direction;
    let tmax = tmax_numerator / direction;
    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

fn trimax(a: f64, b: f64, c: f64) -> f64 {
    a.max(b.max(c))
}

fn trimin(a: f64, b: f64, c: f64) -> f64 {
    a.min(b.min(c))
}

fn intersect(shape: Arc<Shape>, ray: &Ray) -> Option<Intersections> {
    base_shape_intersect(shape, ray, |shape, ray| {
        let (xtmin, xtmax) = check_axis(ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = check_axis(ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = check_axis(ray.origin.z(), ray.direction.z());
        let tmin = trimax(xtmin, ytmin, ztmin);
        let tmax = trimin(xtmax, ytmax, ztmax);
        if tmin > tmax {
            None
        } else {
            Some(Intersections::new(vec![
                Intersection::new(tmin, Arc::clone(&shape)),
                Intersection::new(tmax, Arc::clone(&shape)),
            ]))
        }
    })
}

fn normal_at(shape: Arc<Shape>, point: &Point) -> Vec3D {
    base_shape_normal(shape, point, |_, point| {
        let ax = point.x().abs();
        let ay = point.y().abs();
        let az = point.z().abs();
        let maxc = trimax(ax, ay, az);
        match maxc {
            x if x == ax => vector(point.x(), 0., 0.),
            x if x == ay => vector(0., point.y(), 0.),
            _ => vector(0., 0., point.z()),
        }
    })
}

impl Shape {
    pub fn new_cube(material: Material, transform: Transformation) -> Self {
        Self::new(CUBE, material, transform)
    }

    pub fn default_cube() -> Self {
        Self::new_cube(Material::default(), Transformation::identity())
    }
}
