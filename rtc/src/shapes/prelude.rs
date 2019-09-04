use std::marker::PhantomData;
use std::ops::Index;
use std::sync::Arc;
use std::fmt;

use crate::{
    primitives::{
        approx_eq::{ApproxEq, EPSILON_F64},
        ray::Ray,
        vector::{Point, ScalarProd, Transformation, Vec3D},
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
    // pub self_rc: Option<Rc<Self>>,
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shape {{ transformation: {:?}, inverse_transformation: {:?} , material: {:?}}}", self.transform, self.inverse_transform, self.material)
    }
}

impl Shape {
    pub fn new(transform: Transformation, material: Material, funcs: ShapeFuncs) -> Self {
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

    pub fn modify_transform(&mut self, f: impl Fn(&mut Transformation)) {
        f(&mut self.transform);
        self.inverse_transform = self.transform.invert().unwrap();
    }

    pub fn to_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl ApproxEq for &Shape {
    fn approx_eq(self, other: Self) -> bool {
        self.transform.approx_eq(&other.transform)
            && self.material.approx_eq(&other.material)
    }
}
