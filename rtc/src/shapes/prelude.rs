use std::marker::PhantomData;
use std::ops::Index;

use crate::{
    primitives::{
        approx_eq::{ApproxEq, EPSILON_F64},
        ray::Ray,
        vector::{Point, ScalarProd, Transformation, Vec3D},
    },
    shading::Material,
};

pub use super::{Intersection, Intersections};

/// A general shape
/// Specialize with some empty type that implements IsShape
/// e.g.
/// ```
/// pub enum Sphere {}
/// impl IsShape for Sphere {}
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Shape<T>
where
    T: IsShape,
{
    _shape: PhantomData<T>,
    transform: Transformation,
    inverse_transform: Transformation,
    pub material: Material,
}

impl<T: IsShape> ApproxEq for &Shape<T> {
    fn approx_eq(self, other: Self) -> bool {
        self.transform.approx_eq(&other.transform) && self.material.approx_eq(&other.material)
    }
}

impl<T: IsShape> Default for Shape<T> {
    fn default() -> Self {
        Self::new(Transformation::identity(), Material::default())
    }
}

impl<T: IsShape> Shape<T> {
    pub fn new(transform: Transformation, material: Material) -> Self {
        let inverse_transform = transform
            .invert()
            .expect("Encountered non invertible matrix.");
        Shape {
            _shape: PhantomData,
            transform,
            inverse_transform,
            material: material,
        }
    }

    pub fn transform<'a>(&'a self) -> &'a Transformation {
        &self.transform
    }

    pub fn inverse_transform<'a>(&'a self) -> &'a Transformation {
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
}

/// Marker trait for shapes
/// Automatically implemented for all T where Shape<T> is Render
pub trait IsShape {}

/// Trait for shapes that are renderable
/// should only be implemented for specializations of `Shape<T>`.
pub trait Render<'a, T>
where
    Self: Sized,
    T: IsShape,
{
    /// Find all intersections of a shape with a ray if there are some
    fn intersect(&'a self, ray: &Ray) -> Option<Intersections<'a, T>>;
    /// Calculate the normal vector at any point on the shape
    fn normal_at(&self, point: &Point) -> Vec3D;
}

impl<'a, T> IsShape for T
where Shape<T>: Render<'a, T> {}
