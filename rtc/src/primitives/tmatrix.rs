use std::fmt;
use std::ops::{Index, IndexMut, Mul};

use crate::utils::*;

use super::approx_eq::ApproxEq;

/// Matrix MxN with internal representation in row major order
/// e.g. 4x3
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// 8 7 6
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Matrix<T, M: Nat, N: Nat> {
    m: M,
    n: N,
    pub data: Vec<T>,
}

#[macro_export]
macro_rules! matrix {
    ( $m:ty, $n:ty => $( $( $val:expr ),+ );* ) => {
        {
            use $crate::primitives::tmatrix::Matrix;
            let data = vec![ $( $($val),+ ),* ];
            let mut m = Matrix::<_, $m, $n>::new();
            m.data = data;
            let m = m;
            m
        }
    }
}

impl<T: Default + Clone, M: Nat, N: Nat> Matrix<T, M, N> {
    pub fn new() -> Self {
        Matrix {
            data: vec![Default::default(); M::val() * N::val()],
            m: M::default(),
            n: N::default(),
        }
    }

    /// Iterate over all elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Iterate over the ith row of the matrix
    pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &T> {
        self.iter().skip(i * N::val()).take(N::val())
    }

    /// Iterate over the jth coloumn of the matrix
    pub fn iter_col(&self, j: usize) -> impl Iterator<Item = &T> {
        self.iter().skip(j).step_by(N::val())
    }

    /// Iterate over all rows of the matrix
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..M::val()).map(move |i| self.iter_row(i))
    }

    /// Iterate over all rows of the matrix
    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..N::val()).map(move |j| self.iter_col(j))
    }
}

impl<T, M: Nat, N: Nat> fmt::Debug for Matrix<T, M, N>
where
    T: fmt::Debug + Default + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}", M::val(), N::val())?;
        for row in self.iter_rows() {
            writeln!(f, "    {:?}", row.collect::<Vec<_>>())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn iter() {
        let a = matrix![ N2, N3 =>
            1, 2, 3;
            5, 6, 7
        ];
        let mut b = a.iter();
        assert_eq!(b.next(), Some(&1));
        assert_eq!(b.next(), Some(&2));
        assert_eq!(b.next(), Some(&3));
        assert_eq!(b.next(), Some(&5));
        assert_eq!(b.next(), Some(&6));
        assert_eq!(b.next(), Some(&7));
        assert_eq!(b.next(), None);
    }
}

/*
#[derive(Clone, PartialEq, PartialOrd)]
/// Matrix MxN with internal representation in row major order
/// e.g. 4x3
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// 8 7 6
pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: Vec<T>,
}

#[macro_export]
macro_rules! matrix {
    ( $m:expr, $n:expr => $( $( $val:expr ),+ );* ) => {
        {
            use $crate::primitives::tmatrix::Matrix;
            let data = vec![ $( $($val),+ ),* ];
            let mut m = Matrix::<_, $m, $n>::new();
            m.data = data;
            let m = m;
            m
        }
    }
}

impl<T: Default + Clone, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    pub fn new() -> Self {
        Matrix {
            data: vec![Default::default(); M * N],
        }
    }

    /// Iterate over all elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Iterate over the ith row of the matrix
    pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &T> {
        self.iter().skip(i * N).take(N)
    }

    /// Iterate over the jth coloumn of the matrix
    pub fn iter_col(&self, j: usize) -> impl Iterator<Item = &T> {
        self.iter().skip(j).step_by(N)
    }

    /// Iterate over all rows of the matrix
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..M).map(move |i| self.iter_row(i))
    }

    /// Iterate over all rows of the matrix
    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..N).map(move |j| self.iter_col(j))
    }
}

impl<T, const M: usize, const N: usize> fmt::Debug for Matrix<T, { M }, { N }>
where
    T: fmt::Debug + Default + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}", M, N)?;
        for row in self.iter_rows() {
            writeln!(f, "    {:?}", row.collect::<Vec<_>>())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn iter() {
        /*
        let a = matrix![ 2, 3 =>
            1, 2, 3;
            5, 6, 7
        ].iter();
        for x in a {
            dbg!(x);
        }
        assert!(false);
        */
        let mut a = Matrix::<usize, 2, 3>::new();
        a.data = vec![1, 2, 3, 5, 6, 7];
    }
}
*/