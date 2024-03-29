use num_traits::Num;

use crate::utils::typelevel_nums::*;

use super::{tmatrix::Matrix, transformation_matrices::Matrix4x4};

/// A general 4D Vector
pub type Vec4D<T> = Matrix<T, N4, N1>;

/// A Vector in 3D Space - the fourth dimension identifies this as a 3D vector
pub type Vec3D = Vec4D<f64>;

/// A Point in 3D Space implemented as a vector - the fourth dimension identifies this as a point
pub type Point = Vec4D<f64>;

/// A transformation matrix for Vec4Ds
pub type Transformation = Matrix4x4<f64>;

#[macro_export]
macro_rules! vec4 {
    ( $x:expr, $y:expr, $z:expr, $w:expr ) => {{
        Vec4D::from(vec![$x, $y, $z, $w])
    }};
}

/// Construct a new vector in 3D space
pub fn vector<T: Num>(x: T, y: T, z: T) -> Vec4D<T> {
    vec4![x, y, z, T::zero()]
}

/// Construct a new point in 3D space
pub fn point<T: Num>(x: T, y: T, z: T) -> Vec4D<T> {
    vec4![x, y, z, T::one()]
}

impl Point {
    pub fn origin() -> Self {
        point(0., 0., 0.)
    }
}

impl Vec3D {
    /// Reflect a vector along a normal vector ("Bounce it")
    pub fn reflect(&self, normal: &Self) -> Self {
        self - (normal * 2.0 * self.scalar_prod(normal))
    }
}

impl<T: Copy> Vec4D<T> {
    pub fn x(&self) -> T {
        self[(0, 0)]
    }

    pub fn y(&self) -> T {
        self[(1, 0)]
    }

    pub fn z(&self) -> T {
        self[(2, 0)]
    }

    pub fn w(&self) -> T {
        self[(3, 0)]
    }

    pub fn set_x(&mut self, v: T) {
        self[(0, 0)] = v;
    }

    pub fn set_y(&mut self, v: T) {
        self[(1, 0)] = v;
    }

    pub fn set_z(&mut self, v: T) {
        self[(2, 0)] = v;
    }

    pub fn set_w(&mut self, v: T) {
        self[(3, 0)] = v;
    }
}

pub trait ScalarProd<Rhs = Self> {
    type Output;
    /// Calculate the dot product / scalar product of two vectors
    fn scalar_prod(self, other: Rhs) -> Self::Output;
}

/// v1 * v2
impl<T: Num + Copy + Default + std::iter::Sum<T>> ScalarProd for Vec4D<T> {
    type Output = T;
    fn scalar_prod(self, other: Self) -> Self::Output {
        let m = self.transpose() * other;
        m.as_scalar()
    }
}

/// v1 * &v2
impl<T: Num + Copy + Default + std::iter::Sum<T>> ScalarProd<&Self> for Vec4D<T> {
    type Output = T;
    fn scalar_prod(self, other: &Self) -> Self::Output {
        let m = self.transpose() * other.clone();
        m.as_scalar()
    }
}

/// &v1 * v2
impl<T: Num + Copy + Default + std::iter::Sum<T>> ScalarProd<Vec4D<T>> for &Vec4D<T> {
    type Output = T;
    fn scalar_prod(self, other: Vec4D<T>) -> Self::Output {
        let m = self.transpose() * other;
        m.as_scalar()
    }
}

/// &v1 * &v2
impl<T: Num + Copy + Default + std::iter::Sum<T>> ScalarProd for &Vec4D<T> {
    type Output = T;
    fn scalar_prod(self, other: Self) -> Self::Output {
        let m = self.transpose() * other.clone();
        m.as_scalar()
    }
}

pub trait CrossProd<Rhs = Self> {
    type Output;
    /// Cross product between two vectors
    fn cross(self, other: Rhs) -> Self::Output;
}

/// v1 x v2
impl<T: Num + Copy + Default + std::iter::Sum<T>> CrossProd for Vec4D<T> {
    type Output = Vec4D<T>;
    fn cross(self, other: Self) -> Self::Output {
        vector(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

/// v1 x &v2
impl<T: Num + Copy + Default + std::iter::Sum<T>> CrossProd<&Self> for Vec4D<T> {
    type Output = Vec4D<T>;
    fn cross(self, other: &Self) -> Self::Output {
        vector(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

/// &v1 x v2
impl<T: Num + Copy + Default + std::iter::Sum<T>> CrossProd<Vec4D<T>> for &Vec4D<T> {
    type Output = Vec4D<T>;
    fn cross(self, other: Vec4D<T>) -> Self::Output {
        vector(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

/// &v1 x &v2
impl<T: Num + Copy + Default + std::iter::Sum<T>> CrossProd for &Vec4D<T> {
    type Output = Vec4D<T>;
    fn cross(self, other: Self) -> Self::Output {
        vector(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

impl<T: Num + Copy + Default + std::iter::Sum<T>> Vec4D<T> {
    /// x direction unit vector
    pub fn unit_x() -> Self {
        vector(T::one(), T::zero(), T::zero())
    }
    /// y direction unit vector
    pub fn unit_y() -> Self {
        vector(T::zero(), T::one(), T::zero())
    }

    /// z direction unit vector
    pub fn unit_z() -> Self {
        vector(T::zero(), T::zero(), T::one())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_approx_eq, matrix, primitives::approx_eq::ApproxEq};

    #[test]
    fn scalar_prod() {
        let v1 = vector(1.0, -2.0, 3.0);
        let v2 = vector(5.0, 8.2, -3.1);
        assert!(v1.scalar_prod(v2).approx_eq(5.0 - 16.4 - 9.3));
        let v1 = vector(1.0, -2.0, 3.0);
        let v2 = vector(5.0, 8.2, -3.1);
        assert!(v1.scalar_prod(v2).approx_eq(5.0 - 16.4 - 9.3));
    }

    #[test]
    fn mag() {
        assert!(1.0.approx_eq(vector(1.0, 0.0, 0.0).mag()));
        assert!(1.0.approx_eq(vector(0.0, 1.0, 0.0).mag()));
        assert!(1.0.approx_eq(vector(0.0, 0.0, 1.0).mag()));
        assert!(1.0.approx_eq(vector(0.0, 0.0, -1.0).mag()));
        assert!((2.0_f64.sqrt()).approx_eq(vector(1.0, 1.0, 0.0).mag()));
        assert!((3.0_f64.sqrt()).approx_eq(vector(1.0, 1.0, 1.0).mag()));
        assert!(5.0.approx_eq(vector(5.0, 0.0, 0.0).mag()));
    }

    #[test]
    fn unit() {
        assert!(vector(1.0, 0.0, 0.0).approx_eq(&vector(4.0, 0.0, 0.0).unit()));
        assert!(vector(
            1.0 / 14.0_f64.sqrt(),
            2.0 / 14.0_f64.sqrt(),
            3.0 / 14.0_f64.sqrt()
        )
        .approx_eq(&vector(1.0, 2.0, 3.0).unit()));
        assert!(1.0_f64.approx_eq(vector(1.0, 2.0, 3.0).unit().mag()));
    }

    #[test]
    fn cross() {
        assert!(Vec4D::<f64>::unit_z().approx_eq(&Vec4D::unit_x().cross(Vec4D::unit_y())));
        assert!(Vec4D::<f32>::unit_z().approx_eq(&Vec4D::unit_x().cross(Vec4D::unit_y())));
    }

    #[test]
    fn mul() {
        let a = matrix![ N4, N4 =>
            1 0 0 4;
            0 1 0 8;
            0 0 1 6;
            0 0 0 1
        ];
        let b = point(2, 3, 4);
        let c = vec4![6, 11, 10, 1];
        assert_eq!(a.clone() * b, c);
        let b = vector(2, 3, 4);
        let c = vec4![2, 3, 4, 0];
        assert_eq!(a * b, c);
    }

    #[test]
    fn reflect() {
        let v = vector(1., -1., 0.);
        let n = vector(0., 1., 0.);
        let r = v.reflect(&n);
        assert_approx_eq!(r, &vector(1., 1., 0.));
    }

    #[test]
    fn reflect_slanted() {
        let v = vector(0., -1., 0.);
        let a = std::f64::consts::SQRT_2 / 2.0;
        let n = vector(a, a, 0.);
        let r = v.reflect(&n);
        assert_approx_eq!(r, &vector(1., 0., 0.));
    }
}
