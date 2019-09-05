use std::{ops::Index, sync::Arc};

use crate::primitives::{
    approx_eq::{ApproxEq, EPSILON_F64},
    ray::Ray,
    vector::{Point, ScalarProd, Vec3D},
};

use super::prelude::*;

#[derive(Clone, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Arc<Shape>,
}

impl Intersection {
    pub fn new(t: f64, object: Arc<Shape>) -> Self {
        Intersection { t, object }
    }

    pub fn prepare_computations(self, ray: &Ray) -> PreComp {
        let point = ray.position(self.t);
        let eye = -ray.direction.clone();
        let mut normal = (self.object.normal_at)(self.object.clone(), &point);
        let inside = (&normal).scalar_prod(&eye) < 0.;
        if inside {
            normal = -normal;
        }
        let over_point = &point + &normal * EPSILON_F64 * 10.0;
        let reflection = ray.direction.reflect(&normal);
        PreComp::new(point, eye, normal, reflection, self, inside, over_point)
    }
}

impl ApproxEq for Intersection {
    fn approx_eq(self, other: Self) -> bool {
        self.t.approx_eq(other.t) && self.object.approx_eq(&other.object)
    }
}

impl ApproxEq for &Intersection {
    fn approx_eq(self, other: Self) -> bool {
        self.t.approx_eq(other.t) && self.object.approx_eq(&other.object)
    }
}

#[derive(Clone, Debug)]
pub struct Intersections {
    /// Intersections
    is: Vec<Intersection>,
}

impl Intersections {
    pub fn new(is: Vec<Intersection>) -> Self {
        let mut is = is;
        is.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections { is }
    }

    pub fn len(&self) -> usize {
        self.is.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
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

    pub fn iter(&self) -> impl Iterator<Item = &Intersection> {
        self.is.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Intersection> {
        self.is.into_iter()
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, i: usize) -> &Self::Output {
        &self.is[i]
    }
}

impl ApproxEq for &Intersections {
    fn approx_eq(self, other: Self) -> bool {
        self.is
            .iter()
            .zip(other.is.iter())
            .all(|(l, r)| l.approx_eq(r))
    }
}

/// Precomputations of values of interest of some intersection
#[derive(Clone, Debug)]
pub struct PreComp {
    pub point: Point,
    pub eye: Vec3D,
    pub normal: Vec3D,
    pub reflection: Vec3D,
    pub t: f64,
    pub object: Arc<Shape>,
    pub inside: bool,
    pub over_point: Point,
}

impl PreComp {
    pub fn new(
        point: Point,
        eye: Vec3D,
        normal: Vec3D,
        reflection: Vec3D,
        intersection: Intersection,
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
            reflection,
        }
    }
}
