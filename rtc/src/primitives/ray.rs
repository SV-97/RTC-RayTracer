use crate::primitives::vector::{Point, Vec3D};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3D) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        &self.origin + &self.direction * t
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
    fn new() {
        let p = point(1., 2., 3.);
        let v = vector(4., 5., 6.);
        let r = Ray::new(p.clone(), v.clone());
        assert_approx_eq!(r.origin, &p);
        assert_approx_eq!(r.direction, &v);
    }

    #[test]
    fn position() {
        let r = Ray::new(point(2., 3., 4.), vector(1., 0., 0.));
        assert_approx_eq!(r.position(0.), &point(2., 3., 4.));
        assert_approx_eq!(r.position(1.), &point(3., 3., 4.));
        assert_approx_eq!(r.position(-1.), &point(1., 3., 4.));
        assert_approx_eq!(r.position(2.5), &point(4.5, 3., 4.));
    }
}
