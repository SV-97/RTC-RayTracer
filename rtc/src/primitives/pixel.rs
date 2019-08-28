use std::convert::From;
use std::ops::{Add, Mul, Sub};

use super::approx_eq::{ApproxEq, EPSILON_F32};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
/// A 32-bit pixel in RGB colour space
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Pixel {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Pixel { r, g, b }
    }

    /// Apply a function to each element of the pixel creating a new pixel.
    pub fn map(self, f: impl Fn(f32) -> f32) -> Self {
        Pixel::new_rgb(f(self.r), f(self.g), f(self.b))
    }

    /// Create a new pixel by combining two pixels elementwise using f.
    pub fn combine(self, other: Self, f: impl Fn(f32, f32) -> f32) -> Self {
        Pixel::new_rgb(f(self.r, other.r), f(self.g, other.g), f(self.b, other.b))
    }

    /// Blend two colours / compute the hadamard product of two colours
    pub fn blend(self, other: Self) -> Self {
        self * other
    }

    pub fn abs(self) -> Self {
        self.map(f32::abs)
    }

    pub fn red() -> Self {
        Pixel::new_rgb(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Pixel::new_rgb(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Pixel::new_rgb(0.0, 0.0, 1.0)
    }

    pub fn white() -> Self {
        Pixel::new_rgb(1.0, 1.0, 1.0)
    }
}

impl From<(u8, u8, u8)> for Pixel {
    fn from(bytes: (u8, u8, u8)) -> Self {
        let factor = 255.0_f32.recip();
        Pixel::new_rgb(
            factor * f32::from(bytes.0),
            factor * f32::from(bytes.1),
            factor * f32::from(bytes.2),
        )
    }
}

impl IntoIterator for Pixel {
    type Item = f32;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.r, self.g, self.b].into_iter()
    }
}

impl Add for Pixel {
    type Output = Pixel;
    fn add(self, other: Self) -> Self {
        self.combine(other, Add::add)
    }
}

impl Sub for Pixel {
    type Output = Pixel;
    fn sub(self, other: Self) -> Self {
        self.combine(other, Sub::sub)
    }
}

impl Mul for Pixel {
    type Output = Pixel;
    fn mul(self, other: Self) -> Self {
        self.combine(other, Mul::mul)
    }
}

impl Mul<f32> for Pixel {
    type Output = Pixel;
    fn mul(self, other: f32) -> Self {
        self.map(|c| c * other)
    }
}

impl ApproxEq for Pixel {
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs().into_iter().all(|c| c < EPSILON_F32)
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::new_rgb(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approx_eq() {
        assert!(!Pixel::white().approx_eq(Pixel::red()));
        let c1 = Pixel::new_rgb(0.9, 0.1, 0.000000000000001);
        let c2 = Pixel::new_rgb(0.90000000007, 0.1, 0.0);
        assert!(c1.approx_eq(c2));
    }

    #[test]
    fn add() {
        let c1 = Pixel::new_rgb(0.9, 0.6, 0.75);
        let c2 = Pixel::new_rgb(0.7, 0.1, 0.25);
        assert!((c1 + c2).approx_eq(Pixel::new_rgb(1.6, 0.7, 1.0)));
    }

    #[test]
    fn sub() {
        let c1 = Pixel::new_rgb(0.9, 0.6, 0.75);
        let c2 = Pixel::new_rgb(0.7, 0.1, 0.25);
        assert!((c1 - c2).approx_eq(Pixel::new_rgb(0.2, 0.5, 0.5)));
    }

    #[test]
    fn scalar_mul() {
        let c1 = Pixel::new_rgb(0.2, 0.3, 0.4);
        assert!((c1 * 2.0).approx_eq(Pixel::new_rgb(0.4, 0.6, 0.8)));
    }

    #[test]
    fn blend() {
        let c1 = Pixel::new_rgb(1.0, 0.2, 0.4);
        let c2 = Pixel::new_rgb(0.9, 1.0, 0.1);
        assert!(c1.blend(c2).approx_eq(Pixel::new_rgb(0.9, 0.2, 0.04)));
    }
}
