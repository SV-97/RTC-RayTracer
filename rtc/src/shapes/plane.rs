use std::sync::Arc;

use super::prelude::*;

use crate::{
    primitives::{
        approx_eq::EPSILON_F64,
        ray::Ray,
        vector::{vector, Point, Transformation, Vec3D},
    },
    shading::Material,
};

pub static PLANE: ShapeFuncs = (intersect, normal_at);

fn intersect(shape: Arc<Shape>, ray: &Ray) -> Option<Intersections> {
    base_shape_intersect(shape, ray, |shape, ray2| {
        if ray2.direction.y().abs() < EPSILON_F64 {
            None
        } else {
            let t = -ray2.origin.y() / ray2.direction.y();
            Some(Intersections::new(vec![Intersection::new(t, shape)]))
        }
    })
}

fn normal_at(shape: Arc<Shape>, point: &Point) -> Vec3D {
    base_shape_normal(shape, point, |_, _| vector(0., 1., 0.))
}

impl Shape {
    pub fn new_plane(material: Material, transform: Transformation) -> Self {
        Self::new(PLANE, material, transform)
    }
}
