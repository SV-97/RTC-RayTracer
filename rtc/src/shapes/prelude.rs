use std::cell::Ref;
use std::ops::Index;

use crate::primitives::{approx_eq::ApproxEq, ray::Ray, vector::Transformation};

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
