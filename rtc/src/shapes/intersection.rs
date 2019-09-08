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

    pub fn prepare_computations(&self, ray: &Ray, xs: &Intersections) -> PreComp {
        let point = ray.position(self.t);
        let eye = -ray.direction.clone();
        let mut normal = (self.object.normal_at)(self.object.clone(), &point);
        let inside = (&normal).scalar_prod(&eye) < 0.;
        if inside {
            normal = -normal;
        }
        // 10.0 is a factor that may be tweaked depending on visual artifacts
        let over_point = &point + &normal * EPSILON_F64;
        let under_point = &point - &normal * EPSILON_F64;
        let reflection = ray.direction.reflect(&normal);

        let mut containers: Vec<&Arc<Shape>> = Vec::new();
        let mut n1 = None;
        let mut n2 = None;
        for intersection in xs.iter() {
            // equivalent to self as *const _ == intersection as *const _
            let i_eq_hit = std::ptr::eq(self, intersection);
            if i_eq_hit {
                // dbg!(self, intersection);
                n1 = containers.last().map(|o| o.material.refractive_index);
            }

            // Find the position of the current object in containers
            if let Some(position) = containers
                .iter()
                .position(|&x| Arc::ptr_eq(x, &intersection.object))
            {
                // remove it if it's in there
                containers.remove(position);
            } else {
                // add it if it isn't
                containers.push(&intersection.object);
            }
            if i_eq_hit {
                n2 = containers.last().map(|o| o.material.refractive_index);
                break;
            }
        }
        PreComp::new(
            point,
            eye,
            normal,
            reflection,
            self.clone(),
            inside,
            over_point,
            under_point,
            n1.unwrap_or(1.0),
            n2.unwrap_or(1.0),
        )
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
    pub under_point: Point,
    pub n1: f32,
    pub n2: f32,
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
        under_point: Point,
        n1: f32,
        n2: f32,
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
            n1,
            n2,
            under_point,
        }
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = (&self.eye).scalar_prod(&self.normal);
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) as f64 * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }
            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2) as f64;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
