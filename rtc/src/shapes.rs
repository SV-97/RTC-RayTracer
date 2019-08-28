use std::ops::Index;

use crate::primitives::{
    approx_eq::ApproxEq,
    ray::Ray,
    vector::{point, ScalarProd},
};

pub trait Shape<'a>
where
    Self: Sized,
{
    fn intersect(&'a self, ray: &Ray) -> Option<Intersections<'a, Self>>;
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
        Intersections { is }
    }

    pub fn len(&self) -> usize {
        self.is.len()
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
pub struct Sphere;

impl Default for Sphere {
    fn default() -> Self {
        Sphere {}
    }
}

impl ApproxEq for Sphere {
    fn approx_eq(self, other: Self) -> bool {
        true
    }
}

impl ApproxEq for &Sphere {
    fn approx_eq(self, other: Self) -> bool {
        true
    }
}

impl<'a> Shape<'a> for Sphere {
    fn intersect(&'a self, ray: &Ray) -> Option<Intersections<'a, Self>> {
        let sphere_to_ray = (&ray.origin) - point(0., 0., 0.);
        let a = (&ray.direction).scalar_prod(&ray.direction);
        let b = 2.0 * (&ray.direction).scalar_prod(&sphere_to_ray);
        let c = (&sphere_to_ray).scalar_prod(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let d_sqrt = discriminant.sqrt();
            let ta = 2.0 * a;
            let t1 = (-b - d_sqrt) / ta;
            let t2 = (-b + d_sqrt) / ta;
            let mut v = vec![t1, t2];
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(Intersections::new(
                v.into_iter()
                    .map(|t| Intersection::new(t, self))
                    .collect::<Vec<_>>(),
            ))
        }
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
}
