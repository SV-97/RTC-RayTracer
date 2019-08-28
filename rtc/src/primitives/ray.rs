use crate::primitives::vector::{Point, Transformation, Vec3D};

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

    pub fn transform(&self, transformation: Transformation) -> Self {
        Self::new(
            &transformation * &self.origin,
            &transformation * &self.direction,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq,
        primitives::vector::{point, vector, Transformation},
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

    #[test]
    fn transform_translate() {
        let r = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
        let m = Transformation::new_translation(3., 4., 5.);
        let r2 = r.transform(m);
        assert_approx_eq!(r2.origin, &point(4., 6., 8.));
        assert_approx_eq!(r2.direction, &vector(0., 1., 0.));
    }

    #[test]
    fn transform_scale() {
        let r = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
        let m = Transformation::new_scaling(2., 3., 4.);
        let r2 = r.transform(m);
        assert_approx_eq!(r2.origin, &point(2., 6., 12.));
        assert_approx_eq!(r2.direction, &vector(0., 3., 0.));
    }
}
