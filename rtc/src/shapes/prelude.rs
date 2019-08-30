use std::cell::Ref;
use std::ops::Index;

use crate::{
    primitives::{
        approx_eq::ApproxEq,
        ray::Ray,
        vector::{Point, ScalarProd, Transformation, Vec3D},
    },
    shading::Material,
};

pub trait Shape<'a>
where
    Self: Sized,
{
    /// Set the transformation on a shape
    fn set_transform(&mut self, transformation: Transformation);
    /// Get the transformation of a shape
    fn get_transform(&'a self) -> &'a Transformation;
    /// Get the transformation of a shape mutably
    fn get_transform_mut(&'a mut self) -> &'a mut Transformation;
    /// Find all intersections of a shape with a ray if there are some
    fn intersect(&'a self, ray: &Ray) -> Option<Intersections<'a, Self>>;
    /// Get the inversed transformation - should be cached internally
    fn get_inverse_transform(&'a self) -> Ref<'a, Transformation>;
    /// Calculate the normal vector at any point on the shape
    fn normal_at(&self, point: &Point) -> Vec3D;
    /// Get the material applied to the shape
    fn material(&'a self) -> &'a Material;
    /// Get the material applied to the shape mutably
    fn material_mut(&'a mut self) -> &'a mut Material;
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PreComp<'a, T>
where
    T: Shape<'a>,
{
    pub point: Point,
    pub eye: Vec3D,
    pub normal: Vec3D,
    pub t: f64,
    pub object: &'a T,
    pub inside: bool,
}

impl<'a, T: Shape<'a>> PreComp<'a, T> {
    pub fn new(
        point: Point,
        eye: Vec3D,
        normal: Vec3D,
        intersection: Intersection<'a, T>,
        inside: bool,
    ) -> Self {
        PreComp {
            point,
            eye,
            normal,
            t: intersection.t,
            object: intersection.object,
            inside,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Intersection<'a, T>
where
    T: Shape<'a>,
{
    pub t: f64,
    pub object: &'a T,
}

impl<'a, T> Intersection<'a, T>
where
    T: Shape<'a>,
{
    pub fn new(t: f64, object: &'a T) -> Self {
        Intersection { t, object }
    }

    pub fn prepare_computations(self, ray: &Ray) -> PreComp<'a, T> {
        let point = ray.position(self.t);
        let eye = -ray.direction.clone();
        let mut normal = self.object.normal_at(&point);
        let inside = (&normal).scalar_prod(&eye) < 0.;
        if inside {
            normal = -normal;
        }
        PreComp::new(point, eye, normal, self, inside)
    }
}

impl<'a, T: ApproxEq> ApproxEq for Intersection<'a, T>
where
    T: Shape<'a>,
    &'a T: ApproxEq,
{
    fn approx_eq(self, other: Self) -> bool {
        self.t.approx_eq(other.t) && self.object.approx_eq(other.object)
    }
}

impl<'a, T> ApproxEq for &Intersection<'a, T>
where
    T: Shape<'a>,
    &'a T: ApproxEq,
{
    fn approx_eq(self, other: Self) -> bool {
        self.t.approx_eq(other.t) && self.object.approx_eq(other.object)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Intersections<'a, T>
where
    T: Shape<'a>,
{
    /// Intersections
    is: Vec<Intersection<'a, T>>,
}

impl<'a, T> Intersections<'a, T>
where
    T: Shape<'a>,
{
    pub fn new(is: Vec<Intersection<'a, T>>) -> Self {
        let mut is = is;
        is.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections { is }
    }

    pub fn len(&self) -> usize {
        self.is.len()
    }

    pub fn hit(&self) -> Option<&Intersection<'a, T>> {
        self.is.iter().fold(None, |old, new| {
            if let Some(o) = old {
                if new.t < o.t {
                    Some(new)
                } else {
                    Some(o)
                }
            } else if new.t > 0.0 {
                Some(new)
            } else {
                None
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Intersection<'a, T>> {
        self.is.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Intersection<'a, T>> {
        self.is.into_iter()
    }
}

impl<'a, T> Index<usize> for Intersections<'a, T>
where
    T: Shape<'a>,
{
    type Output = Intersection<'a, T>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.is[i]
    }
}

impl<'a, T> ApproxEq for &Intersections<'a, T>
where
    T: Shape<'a>,
    &'a T: ApproxEq,
{
    fn approx_eq(self, other: Self) -> bool {
        self.is
            .iter()
            .zip(other.is.iter())
            .all(|(l, r)| l.approx_eq(r))
    }
}
