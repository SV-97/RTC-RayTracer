//! Matrix Transformations
#![allow(clippy::many_single_char_names)]

use num_traits::{Float, Num};

use std::mem::replace;

use crate::{matrix, utils::typelevel_nums::*};

use super::tmatrix::Matrix;
use super::vector::Vec4D;

pub type Matrix4x4<T> = Matrix<T, N4, N4>;

impl<T: Num + Copy> Matrix4x4<T> {
    /// Translate in space
    pub fn new_translation(x: T, y: T, z: T) -> Self {
        let n = T::zero();
        let o = T::one();
        matrix![ N4, N4 =>
            o n n x;
            n o n y;
            n n o z;
            n n n o
        ]
    }

    /// Scale (in relation to the origin)
    pub fn new_scaling(x: T, y: T, z: T) -> Self {
        let n = T::zero();
        let o = T::one();
        matrix![ N4, N4 =>
            x n n n;
            n y n n;
            n n z n;
            n n n o
        ]
    }

    /// Shear: x in proportion to y, x in proportion to z
    pub fn new_shear(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self {
        let n = T::zero();
        let o = T::one();
        matrix![ N4, N4 =>
             o xy xz n;
            yx  o yz n;
            zx zy  o n;
             n  n  n o
        ]
    }
}

impl<T: Float + Copy> Matrix4x4<T> {
    /// Rotate around the x axis by r radians
    pub fn new_x_rotation(r: T) -> Self {
        let n = T::zero();
        let o = T::one();
        let c = r.cos();
        let s = r.sin();
        matrix![ N4, N4 =>
            o, n,  n, n;
            n, c, -s, n;
            n, s,  c, n;
            n, n,  n, o
        ]
    }

    /// Rotate around the y axis by r radians
    pub fn new_y_rotation(r: T) -> Self {
        let n = T::zero();
        let o = T::one();
        let c = r.cos();
        let s = r.sin();
        matrix![ N4, N4 =>
             c, n, s, n;
             n, o, n, n;
            -s, n, c, n;
             n, n, n, o
        ]
    }

    /// Rotate around the z axis by r radians
    pub fn new_z_rotation(r: T) -> Self {
        let n = T::zero();
        let o = T::one();
        let c = r.cos();
        let s = r.sin();
        matrix![ N4, N4 =>
            c, -s, n, n;
            s,  c, n, n;
            n,  n, o, n;
            n,  n, n, o
        ]
    }
}

impl<T> Vec4D<T>
where
    T: Num + Default + Copy + std::iter::Sum<T>,
{
    /// Translate in space
    pub fn translate(&self, x: T, y: T, z: T) -> Self {
        let a = Matrix::new_translation(x, y, z);
        a * self.clone()
    }

    /// Scale (in relation to the origin)
    pub fn scale(&self, x: T, y: T, z: T) -> Self {
        let a = Matrix::new_scaling(x, y, z);
        a * self.clone()
    }

    /// Shear: x in proportion to y, x in proportion to z
    pub fn shear(&self, xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self {
        let a = Matrix::new_shear(xy, xz, yx, yz, zx, zy);
        a * self.clone()
    }
}

impl<T> Vec4D<T>
where
    T: Float + Default + Copy + std::iter::Sum<T>,
{
    /// Rotate around the x axis by r radians
    pub fn rotate_x(&self, r: T) -> Self {
        let a = Matrix::new_x_rotation(r);
        a * self.clone()
    }

    /// Rotate around the y axis by r radians
    pub fn rotate_y(&self, r: T) -> Self {
        let a = Matrix::new_y_rotation(r);
        a * self.clone()
    }

    /// Rotate around the x axis by z radians
    pub fn rotate_z(&self, r: T) -> Self {
        let a = Matrix::new_z_rotation(r);
        a * self.clone()
    }
}

impl<T> Vec4D<T>
where
    T: Num + Default + Copy + std::iter::Sum<T>,
{
    /// Translate in space
    pub fn translate_mut(&mut self, x: T, y: T, z: T) -> &mut Self {
        let new = Matrix::new_translation(x, y, z) * &*self;
        replace(self, new);
        self
    }

    /// Scale (in relation to the origin)
    pub fn scale_mut(&mut self, x: T, y: T, z: T) -> &mut Self {
        let new = Matrix::new_scaling(x, y, z) * &*self;
        replace(self, new);
        self
    }

    /// Shear: x in proportion to y, x in proportion to z
    pub fn shear_mut(&mut self, xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> &mut Self {
        let new = Matrix::new_shear(xy, xz, yx, yz, zx, zy) * &*self;
        replace(self, new);
        self
    }
}


impl<T> Vec4D<T>
where
    T: Float + Default + Copy + std::iter::Sum<T>,
{
    /// Rotate around the x axis by r radians
    pub fn rotate_x_mut(&mut self, r: T) -> &mut Self {
        let new = Matrix::new_x_rotation(r) * &*self;
        replace(self, new);
        self
    }

    /// Rotate around the y axis by r radians
    pub fn rotate_y_mut(&mut self, r: T) -> &mut Self {
        let new = Matrix::new_y_rotation(r) * &*self;
        replace(self, new);
        self
    }

    /// Rotate around the x axis by z radians
    pub fn rotate_z_mut(&mut self, r: T) -> &mut Self {
        let new = Matrix::new_z_rotation(r) * &*self;
        replace(self, new);
        self
    }
}

impl<T> Matrix4x4<T>
where
    T: Float + Default + Copy + std::iter::Sum<T>,
{
    /// Rotate around the x axis by r radians
    pub fn and_rotate_x(self, r: T) -> Self {
        Matrix::new_x_rotation(r) * self
    }

    /// Rotate around the y axis by r radians
    pub fn and_rotate_y(self, r: T) -> Self {
        Matrix::new_y_rotation(r) * self
    }

    /// Rotate around the x axis by z radians
    pub fn and_rotate_z(self, r: T) -> Self {
        Matrix::new_z_rotation(r) * self
    }
}

impl<T> Matrix4x4<T>
where
    T: Num + Default + Copy + std::iter::Sum<T>,
{
    /// Translate in space
    pub fn and_translate(self, x: T, y: T, z: T) -> Self {
        Matrix::new_translation(x, y, z) * self
    }

    /// Scale (in relation to the origin)
    pub fn and_scale(self, x: T, y: T, z: T) -> Self {
        Matrix::new_scaling(x, y, z) * self
    }

    /// Shear: x in proportion to y, x in proportion to z
    pub fn and_shear(self, xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self {
        Matrix::new_shear(xy, xz, yx, yz, zx, zy) * self
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::vector::{point, vector};
    use std::f64::consts;

    use super::super::approx_eq::ApproxEq;
    use super::*;
    #[test]
    fn translate_point() {
        let p = point(-3., 4., 5.);
        assert!(p.translate(5., -3., 2.).approx_eq(&point(2., 1., 7.)));
        let t = Matrix::new_translation(5., -3., 2.).invert().unwrap();
        assert!((t * p).approx_eq(&point(-8., 7., 3.)));
    }

    #[test]
    fn translate_vec() {
        let p = vector(-3., 4., 5.);
        assert!(p.translate(5., -3., 2.).approx_eq(&vector(-3., 4., 5.)));
    }

    #[test]
    fn scale_point() {
        let p = point(-4., 6., 8.);
        assert!(p.scale(2., 3., 4.).approx_eq(&point(-8., 18., 32.)));
    }

    #[test]
    fn scale_vector() {
        let v = vector(-4., 6., 8.);
        assert!(v.scale(2., 3., 4.).approx_eq(&vector(-8., 18., 32.)));
        let t = Matrix::new_scaling(2., 3., 4.).invert().unwrap();
        assert!((t * v).approx_eq(&vector(-2., 2., 2.)));
    }

    #[test]
    fn relection_scale() {
        let p = point(2, 3, 4);
        assert_eq!(p.scale(-1, 1, 1), point(-2, 3, 4));
    }

    #[test]
    fn rotate_x() {
        let p = point(0., 1., 0.);
        let half_quarter = Matrix::new_x_rotation(consts::FRAC_PI_4);
        assert!((half_quarter * p.clone()).approx_eq(&point(
            0.,
            consts::SQRT_2 / 2.,
            consts::SQRT_2 / 2.
        )));
        assert!(p.rotate_x(consts::FRAC_PI_2).approx_eq(&point(0., 0., 1.)));
    }

    #[test]
    fn rotate_y() {
        let p = point(0., 0., 1.);
        let half_quarter = Matrix::new_y_rotation(consts::FRAC_PI_4);
        assert!((half_quarter * p.clone()).approx_eq(&point(
            consts::SQRT_2 / 2.,
            0.,
            consts::SQRT_2 / 2.
        )));
        assert!(p.rotate_y(consts::FRAC_PI_2).approx_eq(&point(1., 0., 0.)));
    }

    #[test]
    fn rotate_z() {
        let p = point(0., 1., 0.);
        let half_quarter = Matrix::new_z_rotation(consts::FRAC_PI_4);
        assert!((half_quarter * p.clone()).approx_eq(&point(
            -consts::SQRT_2 / 2.,
            consts::SQRT_2 / 2.,
            0.
        )));
        assert!(p.rotate_z(consts::FRAC_PI_2).approx_eq(&point(-1., 0., 0.)));
    }

    #[test]
    fn shear() {
        let p = point(2., 3., 4.);
        assert!(p
            .shear(1., 0., 0., 0., 0., 0.)
            .approx_eq(&point(5., 3., 4.)));
        assert!(p
            .shear(0., 1., 0., 0., 0., 0.)
            .approx_eq(&point(6., 3., 4.)));
        assert!(p
            .shear(0., 0., 1., 0., 0., 0.)
            .approx_eq(&point(2., 5., 4.)));
        assert!(p
            .shear(0., 0., 0., 1., 0., 0.)
            .approx_eq(&point(2., 7., 4.)));
        assert!(p
            .shear(0., 0., 0., 0., 1., 0.)
            .approx_eq(&point(2., 3., 6.)));
        assert!(p
            .shear(0., 0., 0., 0., 0., 1.)
            .approx_eq(&point(2., 3., 7.)));
    }
}
