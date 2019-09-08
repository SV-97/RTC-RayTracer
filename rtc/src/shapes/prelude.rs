use std::{fmt, sync::Arc};

use crate::{
    primitives::{
        approx_eq::ApproxEq,
        ray::Ray,
        vector::{Point, Transformation, Vec3D},
    },
    shading::Material,
};

pub use super::{Intersection, Intersections};

pub type ShapeFuncs = (IntersectFunc, NormalAtFunc);
pub type IntersectFunc = fn(Arc<Shape>, &Ray) -> Option<Intersections>;
pub type NormalAtFunc = fn(Arc<Shape>, &Point) -> Vec3D;

/// A general 3D shape
#[derive(Clone)]
pub struct Shape {
    transform: Transformation,
    inverse_transform: Transformation,
    pub material: Material,
    pub intersect: IntersectFunc,
    pub normal_at: NormalAtFunc,
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Shape{{\nintersect: @ {:p},\nnormal_at: @ {:p},\ntransformation: {:?},\ninverse_transformation: {:?},\nmaterial: {:?}\n}}",
            self.intersect as *const (), self.normal_at as *const (), self.transform, self.inverse_transform, self.material
        )
    }
}

impl Shape {
    pub fn new(funcs: ShapeFuncs, material: Material, transform: Transformation) -> Self {
        let (intersect, normal_at) = funcs;
        let inverse_transform = transform
            .invert()
            .expect("Encountered non invertible matrix.");
        Shape {
            transform,
            inverse_transform,
            material,
            intersect,
            normal_at,
        }
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn inverse_transform(&self) -> &Transformation {
        &self.inverse_transform
    }

    pub fn set_transform(&mut self, transformation: Transformation) {
        self.transform = transformation;
        self.inverse_transform = self.transform.invert().unwrap();
    }

    pub fn modify_transform(&mut self, f: impl Fn(&mut Transformation) -> &mut Transformation) {
        f(&mut self.transform);
        self.inverse_transform = self.transform.invert().unwrap();
    }
}

/// Transform a ray to object space, and get all intersections via f_intersect
pub fn base_shape_intersect(
    shape: Arc<Shape>,
    ray: &Ray,
    f_intersect: impl Fn(Arc<Shape>, Ray) -> Option<Intersections>,
) -> Option<Intersections> {
    let inverse = shape.inverse_transform();
    let ray2 = ray.transform(inverse);
    f_intersect(shape, ray2)
}

/// Applies default transformations, calls a function that operates on the object point and
/// transforms the returned object vector into a world space vector
pub fn base_shape_normal(
    shape: Arc<Shape>,
    point: &Point,
    f_object_normal: impl Fn(Arc<Shape>, Point) -> Vec3D,
) -> Vec3D {
    let inverse = shape.inverse_transform();
    let object_point = inverse * point;
    let world_transform = inverse.transpose();
    let object_normal = f_object_normal(shape, object_point);
    let mut out = world_transform * object_normal;
    out.set_w(0.0);
    out.unit()
}

impl ApproxEq for &Shape {
    fn approx_eq(self, other: Self) -> bool {
        self.transform.approx_eq(&other.transform) && self.material.approx_eq(&other.material)
    }
}
