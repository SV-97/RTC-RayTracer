/// Trait that provides equivalence for floating-point based types
pub trait ApproxEq<Rhs = Self> {
    /// Maximum allowed error such that two instances are regarded as being equal.
    fn approx_eq(self, other: Rhs) -> bool;
}

pub const EPSILON_F64: f64 = 0.1e-3;
impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs() < EPSILON_F64
    }
}

pub const EPSILON_F32: f32 = 0.1e-3;
impl ApproxEq for f32 {
    fn approx_eq(self, other: Self) -> bool {
        (self - other).abs() < EPSILON_F32
    }
}

/*
impl<E> ApproxEq<E> for Vec<E>
where E: ApproxEq
{
    const EPSILON: E = E::EPSILON;
    fn approx_eq(self, other: Self) -> bool {
        self.iter().zip(other.iter()).all(|(l, r)| l.approx_eq(*r))
    }
}

impl<'a, E> ApproxEq<E> for &'a Vec<E>
where
    &'a E: ApproxEq,
    E: ApproxEq
{
    const EPSILON: E = E::EPSILON;
    fn approx_eq(self, other: Self) -> bool {
        self.iter().zip(other.iter()).all(|(l, r)| l.approx_eq(r))
    }
}
*/

/// Adaption of assert_eq from the stdlib to work with assert_eq rather than std::ops::Eq::eq
#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => ({
        use $crate::primitives::approx_eq::ApproxEq;
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((left_val).approx_eq(*right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left ≈ right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        use $crate::primitives::approx_eq::ApproxEq;
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !((left_val).approx_eq(*right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left ≈ right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
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

    #[test]
    #[should_panic]
    fn approx_eq_macro_panic() {
        assert_approx_eq!(4.2, 4.3);
    }

    #[test]
    fn approx_eq_macro() {
        assert_approx_eq!(4.2, 4.2000000000000001);
    }
}
