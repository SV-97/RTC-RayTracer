use std::ops::Index;
use std::cell::{Ref, RefCell};

use crate::primitives::{
    approx_eq::ApproxEq,
    ray::Ray,
    vector::{point, ScalarProd, Transformation},
};

pub trait Shape<'a>
where
    Self: Sized,
{
    /// Set the transformation on a shape
    fn set_transform(&mut self, transformation: Transformation);
    /// Get the transformation of a shape
    fn get_transform(&'a self) -> &'a Transformation;
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
    t: f64,
    object: &'a T,
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sphere {
    transformation: Transformation,
    inverse_transformation: RefCell<Option<Transformation>>,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transformation: Transformation::identity(),
            inverse_transformation: RefCell::new(Some(Transformation::identity())),
        }
    }
}

impl ApproxEq for Sphere {
    fn approx_eq(self, other: Self) -> bool {
        self.transformation.approx_eq(&other.transformation)
    }
}

impl ApproxEq for &Sphere {
    fn approx_eq(self, other: Self) -> bool {
        self.transformation.approx_eq(&other.transformation)
    }
}

impl<'a> Shape<'a> for Sphere {
    fn intersect(&'a self, ray: &Ray) -> Option<Intersections<'a, Self>> {
        let mut out = None;
        Ref::map(self.get_inverse_transform(), |inverse| {
            let ray2 = ray.transform(inverse);
            let sphere_to_ray = (&ray2.origin) - point(0., 0., 0.);
            let a = (&ray2.direction).scalar_prod(&ray2.direction);
            let b = 2.0 * (&ray2.direction).scalar_prod(&sphere_to_ray);
            let c = (&sphere_to_ray).scalar_prod(&sphere_to_ray) - 1.0;
            let discriminant = b.powi(2) - 4.0 * a * c;
            if discriminant < 0.0 {
                out = None;
            } else {
                let d_sqrt = discriminant.sqrt();
                let ta = 2.0 * a;
                let t1 = (-b - d_sqrt) / ta;
                let t2 = (-b + d_sqrt) / ta;
                let v = vec![t1, t2];
                out = Some(Intersections::new(
                    v.into_iter()
                        .map(|t| Intersection::new(t, self))
                        .collect::<Vec<_>>(),
                ));
            }
            inverse
        });
        out
    }

    fn set_transform(&mut self, transformation: Transformation) {
        self.transformation = transformation;
        self.inverse_transformation.replace(None);
    }

    fn get_transform(&'a self) -> &'a Transformation {
        &self.transformation
    }

    fn get_inverse_transform(&'a self) -> Ref<'a, Transformation> {
        self.inverse_transformation.borrow_mut().get_or_insert_with(|| self.transformation.invert().expect("Encountered uninvertable matrix"));
        Ref::map(self.inverse_transformation.borrow(), |opt_ref| opt_ref.as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq,
        primitives::vector::{point, vector},
    };

    #[test]
    fn intersect_ray_sphere_2() {
        let s = Sphere::default();
        let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let is = s.intersect(&ray).unwrap();
        assert_eq!(is.len(), 2);
        assert_approx_eq!(is[0], &Intersection::new(4.0, &s));
        assert_approx_eq!(is[1], &Intersection::new(6.0, &s));
    }

    #[test]
    fn intersect_ray_sphere_1() {
        let s = Sphere::default();
        let ray = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
        let is = s.intersect(&ray).unwrap();
        assert_eq!(is.len(), 2);
        assert_approx_eq!(is[0], &Intersection::new(5.0, &s));
        assert_approx_eq!(is[1], &Intersection::new(5.0, &s));
    }

    #[test]
    fn intersect_ray_sphere_inside() {
        let s = Sphere::default();
        let ray = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let is = s.intersect(&ray).unwrap();
        assert_eq!(is.len(), 2);
        assert_approx_eq!(is[0], &Intersection::new(-1.0, &s));
        assert_approx_eq!(is[1], &Intersection::new(1.0, &s));
    }

    #[test]
    fn intersect_ray_sphere_behind() {
        let s = Sphere::default();
        let ray = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let is = s.intersect(&ray).unwrap();
        assert_eq!(is.len(), 2);
        assert_approx_eq!(is[0], &Intersection::new(-6.0, &s));
        assert_approx_eq!(is[1], &Intersection::new(-4.0, &s));
    }

    #[test]
    fn hit_all_positive() {
        let s = Sphere::default();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let is = Intersections::new(vec![i2, i1.clone()]);
        assert_approx_eq!(is.hit().unwrap(), &i1);
    }

    #[test]
    fn hit_some_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);
        let is = Intersections::new(vec![i2.clone(), i1]);
        assert_approx_eq!(is.hit().unwrap(), &i2);
    }

    #[test]
    fn hit_all_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2., &s);
        let i2 = Intersection::new(-1., &s);
        let is = Intersections::new(vec![i2, i1]);
        assert_eq!(is.hit(), None);
    }

    #[test]
    fn hit_always_lowest() {
        let s = Sphere::default();
        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);
        let is = Intersections::new(vec![i1, i2, i3, i4.clone()]);
        assert_approx_eq!(is.hit().unwrap(), &i4);
    }

    #[test]
    fn sphere_transform_get() {
        let s = Sphere::default();
        assert_approx_eq!(s.get_transform(), &Transformation::identity());
    }

    #[test]
    fn sphere_transform_set() {
        let mut s = Sphere::default();
        let t = Transformation::new_translation(2., 3., 4.);
        s.set_transform(t.clone());
        assert_approx_eq!(s.get_transform(), &t);
    }

    #[test]
    fn interset_ray_scaled_sphere() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = Sphere::default();
        let t = Transformation::new_scaling(2., 2., 2.);
        s.set_transform(t.clone());
        let is = s.intersect(&r).unwrap();
        assert_eq!(is.len(), 2);
        assert_approx_eq!(is[0].t, 3.);
        assert_approx_eq!(is[1].t, 7.);
    }

    #[test]
    fn interset_ray_translated_sphere() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = Sphere::default();
        let t = Transformation::new_translation(5., 0., 0.);
        s.set_transform(t.clone());
        let is = s.intersect(&r);
        assert!(is.is_none());
    }
}
