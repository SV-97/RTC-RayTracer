use std::convert::From;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use super::approx_eq::ApproxEq;
use super::point::Point;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
/// A vector in 3D space
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    /// Construct a new Vec3D
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3D { x, y, z }
    }

    /// Create a new Vec3D by combining two Vec3D elementwise using f.
    pub fn combine(self, other: Self, f: impl Fn(f64, f64) -> f64) -> Self {
        Vec3D::new(f(self.x, other.x), f(self.y, other.y), f(self.z, other.z))
    }

    /// Apply a function to each element of Vec3D creating a new Vec3D.
    pub fn map(self, f: impl Fn(f64) -> f64) -> Self {
        Vec3D::new(f(self.x), f(self.y), f(self.z))
    }

    /// Sum all components of a vector
    pub fn sum(self) -> f64 {
        self.fold(Add::add, 0.0)
    }

    /// Folds over the vector in x->y->z
    pub fn fold<T>(self, f: impl Fn(T, f64) -> T, initial: T) -> T {
        f(f(f(initial, self.x), self.y), self.z)
    }

    /// Take absolute value of each element of the vector
    pub fn abs(self) -> Self {
        self.map(f64::abs)
    }

    /// Compute the magnitude/norm/length of the vector
    pub fn mag(self) -> f64 {
        self.map(|x| x.powi(2)).sum().sqrt()
    }

    /// Normalize a vector such that you get a vector with same direction
    /// but a magnitude of 1.
    pub fn unit(self) -> Self {
        self / self.mag()
    }

    /// Calculate the scalar product of two vectors
    pub fn scalar_prod(self, other: Self) -> f64 {
        self.combine(other, Mul::mul).sum()
    }

    /// Cross product between two vectors
    pub fn cross(self, other: Self) -> Self {
        Vec3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// x direction unit vector
    pub fn x() -> Self {
        Vec3D::new(1.0, 0.0, 0.0)
    }
    /// y direction unit vector
    pub fn y() -> Self {
        Vec3D::new(0.0, 1.0, 0.0)
    }

    /// z direction unit vector
    pub fn z() -> Self {
        Vec3D::new(0.0, 0.0, 1.0)
    }
}

impl IntoIterator for Vec3D {
    type Item = f64;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z].into_iter()
    }
}

/// Treat a Vec3D as local vector and convert it to a Point.
impl From<Point> for Vec3D {
    fn from(p: Point) -> Self {
        let Point { x, y, z } = p;
        Vec3D::new(x, y, z)
    }
}

impl ApproxEq<f64> for Vec3D {
    const EPSILON: f64 = f64::EPSILON;
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs().into_iter().all(|c| c < Self::EPSILON)
    }
}

impl Sub for Vec3D {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.combine(other, Sub::sub)
    }
}

impl Add for Vec3D {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self.combine(other, Add::add)
    }
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for Vec3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

impl Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        self.map(|x| x * other)
    }
}

impl Div<f64> for Vec3D {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        self.map(|x| x / other)
    }
}

impl Default for Vec3D {
    fn default() -> Self {
        Vec3D::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct() {
        let v = Vec3D::new(4.3, -4.2, 3.1);
        assert!(v.x.approx_eq(4.3));
        assert!(v.y.approx_eq(-4.2));
        assert!(v.z.approx_eq(3.1));
    }

    #[test]
    fn add() {
        let v1 = Vec3D::new(1.0, -2.0, 3.0);
        let v2 = Vec3D::new(5.0, 8.2, -3.1);
        assert!((v1 + v2).approx_eq(Vec3D::new(6.0, 6.2, -0.1)));
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vec3D::new(1.0, -2.0, 3.0);
        let v2 = Vec3D::new(5.0, 8.2, -3.1);
        v1 += v2;
        assert!(v1.approx_eq(Vec3D::new(6.0, 6.2, -0.1)));
    }

    #[test]
    fn sub() {
        let v1 = Vec3D::new(1.0, -2.0, 3.0);
        let v2 = Vec3D::new(-5.0, -8.2, 3.1);
        assert!((v1 - v2).approx_eq(Vec3D::new(6.0, 6.2, -0.1)));
    }

    #[test]
    fn neg() {
        let v1 = Vec3D::new(1.0, -2.0, 3.0);
        assert!((-v1).approx_eq(Vec3D::new(-1.0, 2.0, -3.0)));
        assert!((-v1).approx_eq(Vec3D::default() - v1));
    }

    #[test]
    fn mul() {
        let v1 = Vec3D::new(1.0, -2.0, 3.0);
        assert!((v1 * 3.5).approx_eq(Vec3D::new(3.5, -7.0, 10.5)));
    }

    #[test]
    fn div() {
        let v1 = Vec3D::new(1.0, -2.0, 3.0);
        assert!((v1 / 2.0).approx_eq(v1 * 0.5));
    }

    #[test]
    fn mag() {
        assert!(1.0.approx_eq(Vec3D::new(1.0, 0.0, 0.0).mag()));
        assert!(1.0.approx_eq(Vec3D::new(0.0, 1.0, 0.0).mag()));
        assert!(1.0.approx_eq(Vec3D::new(0.0, 0.0, 1.0).mag()));
        assert!(1.0.approx_eq(Vec3D::new(0.0, 0.0, -1.0).mag()));
        assert!((2.0_f64.sqrt()).approx_eq(Vec3D::new(1.0, 1.0, 0.0).mag()));
        assert!((3.0_f64.sqrt()).approx_eq(Vec3D::new(1.0, 1.0, 1.0).mag()));
        assert!(5.0.approx_eq(Vec3D::new(5.0, 0.0, 0.0).mag()));
    }

    #[test]
    fn unit() {
        assert!(Vec3D::new(1.0, 0.0, 0.0).approx_eq(Vec3D::new(4.0, 0.0, 0.0).unit()));
        assert!(Vec3D::new(
            1.0 / 14.0_f64.sqrt(),
            2.0 / 14.0_f64.sqrt(),
            3.0 / 14.0_f64.sqrt()
        )
        .approx_eq(Vec3D::new(1.0, 2.0, 3.0).unit()));
        assert!(1.0_f64.approx_eq(Vec3D::new(1.0, 2.0, 3.0).unit().mag()));
    }

    #[test]
    fn sum() {
        let v1 = Vec3D::new(1.0, -2.0, 3.0);
        assert!(v1.sum().approx_eq(2.0));
    }

    #[test]
    fn scalar_prod() {
        let v1 = Vec3D::new(1.0, -2.0, 3.0);
        let v2 = Vec3D::new(5.0, 8.2, -3.1);
        assert!(v1.scalar_prod(v2).approx_eq(5.0 - 16.4 - 9.3));
    }

    #[test]
    fn cross() {
        assert!(Vec3D::z().approx_eq(Vec3D::x().cross(Vec3D::y())));
    }
}
