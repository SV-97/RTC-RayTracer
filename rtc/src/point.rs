use std::convert::From;
use std::ops::{Add, Sub};

use super::approx_eq::ApproxEq;
use super::vector::Vec3D;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn combine<V: Into<Vec3D>>(self, other: V, f: impl Fn(f64, f64) -> f64) -> Self {
        let other = other.into();
        Point::new(f(self.x, other.x), f(self.y, other.y), f(self.z, other.z))
    }
}

impl From<Vec3D> for Point {
    fn from(p: Vec3D) -> Self {
        let Vec3D { x, y, z } = p;
        Point::new(x, y, z)
    }
}

impl ApproxEq for Point {
    const EPSILON: Self = Point {
        x: f64::EPSILON,
        y: f64::EPSILON,
        z: f64::EPSILON,
    };
    fn approx_eq(self, other: Self) -> bool {
        Point::from((self - other).abs()) < Self::EPSILON
    }
}

impl Add<Vec3D> for Point {
    type Output = Self;
    fn add(self, other: Vec3D) -> Self::Output {
        self.combine(other, Add::add)
    }
}

impl Sub<Vec3D> for Point {
    type Output = Self;
    fn sub(self, other: Vec3D) -> Self::Output {
        self.combine(other, Sub::sub)
    }
}

impl Sub<Point> for Point {
    type Output = Vec3D;
    fn sub(self, other: Self) -> Self::Output {
        self.combine(other, Sub::sub).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approx_eq() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert!(p.approx_eq(Point::new(
            4.3000000000001,
            -4.199999999999,
            3.0999999999999
        )));
        assert!(!p.approx_eq(Point::new(4.300001, -4.199999999999, 3.0999999999999)));
    }

    #[test]
    fn construct() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert!(p.x.approx_eq(4.3));
        assert!(p.y.approx_eq(-4.2));
        assert!(p.z.approx_eq(3.1));
    }

    #[test]
    fn add() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let v = Vec3D::new(-2.0, 3.0, 1.0);
        assert!((p1 + v).approx_eq(Point::new(1.0, 1.0, 6.0)));
    }

    #[test]
    fn sub_vec() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let v = Vec3D::new(2.0, -3.0, -1.0);
        assert!((p1 - v).approx_eq(Point::new(1.0, 1.0, 6.0)));
    }

    #[test]
    fn sub_point() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let p2 = Point::new(-3.0, 10.0, 8.0);
        assert!((p2 - p1).approx_eq(Vec3D::new(-6.0, 12.0, 3.0)));
    }
}
