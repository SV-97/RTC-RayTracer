/// Trait that provides equivalence for floating-point based types
pub trait ApproxEq<E = Self, Rhs = Self>
where
    E: Copy,
{
    /// Maximum allowed error such that two instances are regarded as being equal.
    const EPSILON: E;
    fn approx_eq(self, other: Rhs) -> bool;
}

impl ApproxEq for f64 {
    const EPSILON: Self = 0.1e-10;
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs() < Self::EPSILON
    }
}

impl ApproxEq for f32 {
    const EPSILON: Self = 0.1e-5;
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs() < Self::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn approx_eq_f64() {
        assert!(4.2.approx_eq(4.2000000000000001));
        assert!(!1.2.approx_eq(1.1));
    }

    #[test]
    fn approx_eq_f32() {
        assert!(4.2.approx_eq(4.2000000000000001));
        assert!(!1.2.approx_eq(1.1));
    }
}
