use std::sync::Arc;

use super::prelude::*;

use crate::{
    primitives::{
        approx_eq::EPSILON_F64,
        ray::Ray,
        vector::{vector, Point, ScalarProd, Transformation, Vec3D},
    },
    shading::Material,
};

pub static PLANE: ShapeFuncs = (intersect, normal_at);

fn intersect(shape: Arc<Shape>, ray: &Ray) -> Option<Intersections> {
    if ray.direction.y().abs() < EPSILON_F64 {
        None
    } else {
        let t = -ray.origin.y() / ray.direction.y();
        Some(Intersections::new(vec![Intersection::new(t, shape)]))
    }
}

fn normal_at(_: Arc<Shape>, _: &Point) -> Vec3D {
    vector(0., 1., 0.)
}

impl Shape {
    pub fn new_plane(material: Material, transform: Transformation) -> Self {
        Self::new(PLANE, material, transform)
    }
}
