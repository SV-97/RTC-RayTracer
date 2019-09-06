//! Struct definition, construction macro, constructor, printing etc.

use num_traits::Signed;

use std::{cmp, convert::From, fmt, marker::PhantomData};

use crate::{primitives::approx_eq::ApproxEq, utils::typelevel_nums::*};

/// Matrix MxN with internal representation in row major order
/// e.g. 4x3
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// 8 7 6
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Matrix<T, M: Nat, N: Nat> {
    _m: PhantomData<M>,
    _n: PhantomData<N>,
    pub data: Vec<T>,
}

/// Create a new matrix of specified size
/// Verifies that the number of values matches the size
/// # Example
/// ```
/// let a = matrix![ N4, N3 =>
///     1   2  3;
///     4 {-5} 6;
///     7   8  9_isize
/// ];
/// ```
/// Creates a new 4x3 matrix of isizes
#[macro_export]
macro_rules! matrix {
    ( $m:ty, $n:ty => $( $( $val:expr ),+ );* ) => {
        {
            type M = $m;
            type N = $n;
            use $crate::primitives::tmatrix::Matrix;
            let data = vec![ $( vec![$($val),+] ),* ];
            assert_eq!(data.len(), M::val());
            assert_eq!(data[0].len(), N::val());
            let flattened = data.into_iter().map(|arr| arr.into_iter()).flatten().collect::<Vec<_>>();
            assert_eq!(flattened.len(), M::val() * N::val());
            Matrix::<_, M, N>::from(flattened)
        }
    };
    ( $m:ty, $n:ty => $( $( $val:expr )+ );* ) => {
        {
            type M = $m;
            type N = $n;
            use $crate::primitives::tmatrix::Matrix;
            let data = vec![ $( vec![$($val),+] ),* ];
            assert_eq!(data.len(), M::val());
            assert_eq!(data[0].len(), N::val());
            let flattened = data.into_iter().map(|arr| arr.into_iter()).flatten().collect::<Vec<_>>();
            assert_eq!(flattened.len(), M::val() * N::val());
            Matrix::<_, M, N>::from(flattened)
        }
    }
}

impl<T, M, N> From<Vec<T>> for Matrix<T, M, N>
where
    M: Nat + Val,
    N: Nat + Val,
{
    fn from(v: Vec<T>) -> Self {
        assert_eq!(v.len(), M::val() * N::val());
        Matrix {
            data: v,
            _m: PhantomData,
            _n: PhantomData,
        }
    }
}

impl<T: Default + Clone, M: Nat + Val, N: Nat + Val> Matrix<T, M, N> {
    pub fn new() -> Self {
        Matrix {
            data: vec![Default::default(); M::val() * N::val()],
            _m: PhantomData,
            _n: PhantomData,
        }
    }
}

impl<T: Default + Clone, M: Nat + Val, N: Nat + Val> Default for Matrix<T, M, N> {
    fn default() -> Self {
        Matrix {
            data: vec![Default::default(); M::val() * N::val()],
            _m: PhantomData,
            _n: PhantomData,
        }
    }
}

impl<T, M: Nat + Val, N: Nat + Val> Matrix<T, M, N> {
    /// Create a new matrix with uninitialized vector
    pub fn new_uninitialized() -> Self {
        Matrix {
            data: vec![],
            _m: PhantomData,
            _n: PhantomData,
        }
    }

    /// Height of the matrix
    pub fn height(&self) -> usize {
        M::val()
    }

    /// width of the matrix
    pub fn width(&self) -> usize {
        N::val()
    }
}

impl<T, M: Nat + Val, N: Nat + Val> fmt::Debug for Matrix<T, M, N>
where
    T: fmt::Debug + Default + Clone + Signed + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}", M::val(), N::val())?;
        let (len, rows) = self
            .iter_rows()
            .map(|row| {
                row.map(|x| {
                    let s = if x >= &T::zero() {
                        format!(" {:?}", x.abs()) // abs to convert -0.0 to 0.0 because fuck that shit
                    } else {
                        format!("{:?}", x)
                    };
                    (s.len(), s)
                })
                .fold((0, vec![]), |(len, mut acc), (s_len, s)| {
                    let new_len = cmp::max(len, s_len);
                    acc.push(s);
                    (new_len, acc)
                })
            })
            .fold((0, vec![]), |(len, mut acc), (row_len, row)| {
                let new_len = cmp::max(len, row_len);
                acc.push(row);
                (new_len, acc)
            });
        let out = rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|s| format!("{:width$}", s, width = len))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .map(|row| format!("    |{} |", row))
            .collect::<Vec<String>>()
            .join("\n");
        writeln!(f, "{}", out)?;
        Ok(())
    }
}

impl<T: ApproxEq + Copy, M: Nat + Val, N: Nat + Val> ApproxEq for &Matrix<T, M, N> {
    fn approx_eq(self, other: Self) -> bool {
        self.iter().zip(other.iter()).all(|(l, r)| l.approx_eq(*r))
    }
}
