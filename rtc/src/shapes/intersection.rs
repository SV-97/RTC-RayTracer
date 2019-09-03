use std::ops::Index;

use crate::{
    primitives::{
        approx_eq::{ApproxEq, EPSILON_F64},
        ray::Ray,
        vector::{Point, ScalarProd, Transformation, Vec3D},
    },
    shading::Material,
};

use super::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Intersection<'a, T>
where
    T: Render<'a>,
{
    pub t: f64,
    pub object: &'a T,
}

impl<'a, T> Intersection<'a, Shape<T>>
where
    T: IsShape,
    Shape<T>: Render<'a>,
{
    pub fn new(t: f64, object: &'a Shape<T>) -> Self {
        Intersection { t, object }
    }

    pub fn prepare_computations(self, ray: &Ray) -> PreComp<'a, Shape<T>> {
        let point = ray.position(self.t);
        let eye = -ray.direction.clone();
        let mut normal = self.object.normal_at(&point);
        let inside = (&normal).scalar_prod(&eye) < 0.;
        if inside {
            normal = -normal;
        }
        let over_point = &point + &normal * EPSILON_F64;
        PreComp::new(point, eye, normal, self, inside, over_point)
    }
}

impl<'a, T> ApproxEq for Intersection<'a, Shape<T>>
where
    T: IsShape,
    &'a Shape<T>: ApproxEq,
{
    fn approx_eq(self, other: Self) -> bool {
        self.t.approx_eq(other.t) && self.object.approx_eq(other.object)
    }
}

impl<'a, T> ApproxEq for &Intersection<'a, Shape<T>>
where
    T: IsShape,
    &'a Shape<T>: ApproxEq,
{
    fn approx_eq(self, other: Self) -> bool {
        self.t.approx_eq(other.t) && self.object.approx_eq(other.object)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Intersections<'a, T>
where
    T: Render<'a>,
{
    /// Intersections
    is: Vec<Intersection<'a, T>>,
}

impl<'a, T> Intersections<'a, Shape<T>>
where
    T: IsShape,
    Shape<T>: Render<'a>,
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
    T: IsShape,
{
    type Output = Intersection<'a, T>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.is[i]
    }
}

impl<'a, T> ApproxEq for &Intersections<'a, T>
where
    T: IsShape,
    &'a Shape<T>: ApproxEq,
{
    fn approx_eq(self, other: Self) -> bool {
        self.is
            .iter()
            .zip(other.is.iter())
            .all(|(l, r)| l.approx_eq(r))
    }
}

/// Precomputations of values of interest of some intersection
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PreComp<'a, T>
where
    T: IsShape,
{
    pub point: Point,
    pub eye: Vec3D,
    pub normal: Vec3D,
    pub t: f64,
    pub object: &'a Shape<T>,
    pub inside: bool,
    pub over_point: Point,
}

impl<'a, T: IsShape> PreComp<'a, T> {
    pub fn new(
        point: Point,
        eye: Vec3D,
        normal: Vec3D,
        intersection: Intersection<'a, T>,
        inside: bool,
        over_point: Point,
    ) -> Self {
        PreComp {
            point,
            eye,
            normal,
            t: intersection.t,
            object: intersection.object,
            inside,
            over_point,
        }
    }
}
