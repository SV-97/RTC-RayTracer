use crate::primitives::{point::Point, vector::Vec3D, tmatrix::Matrix};


#[cfg(test)]
mod tests {
    #[test]
    fn translation() {
        let t = translation(5., -3., 2.);
        let p = Point::new(-3., 4., 5.);
        assert!((t * p).approx_eq(Point::new(2., 1., 7.)));
    }

    fn inverse_translation() {
        let t = translation(5., -3., 2.);
        let p = Point::new(-3., 4., 5.);
        assert!((t * p).approx_eq(Point::new(-8., 7., 3.)));
    }
}