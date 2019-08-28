//! A bit more advanced math operations

use num::Integer;
use num_traits::{Num, NumAssignOps, Signed};

use crate::utils::typelevel_nums::*;

use super::prelude::*;

impl<T, M, N> Matrix<T, Succ<M>, N>
where
    T: Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    /// Remove the i'th row
    pub fn remove_row(&self, i: usize) -> Matrix<T, M, N> {
        let mut m = Matrix::new_uninitialized();
        m.data = self
            .iter_rows()
            .enumerate()
            .filter_map(|(x, row)| if x != i { Some(row) } else { None })
            .flatten()
            .copied()
            .collect::<Vec<T>>();
        m
    }
}

impl<T, M, N> Matrix<T, M, Succ<N>>
where
    T: Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    /// Remove the j'th col
    pub fn remove_col(&self, j: usize) -> Matrix<T, M, N> {
        let mut m = Matrix::new_uninitialized();
        m.data = self
            .iter_cols()
            .enumerate()
            .filter_map(|(x, col)| if x != j { Some(col) } else { None })
            .fold(None, |old: Option<Vec<Vec<_>>>, new| match old {
                // zip all the iterators
                // TODO Maybe test another datastructure here - VecDeque or LinkedList or smth
                // Could also transpose -> remove row -> transpose, if that is more efficient
                Some(it) => Some(
                    it.into_iter()
                        .zip(new)
                        .map(|(mut v, elem)| {
                            v.push(elem);
                            v
                        })
                        .collect::<Vec<Vec<_>>>(),
                ),
                None => Some(new.map(|x| vec![x]).collect::<Vec<_>>()),
            })
            .unwrap()
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<T>>();
        m
    }
}

impl<T, M, N> Matrix<T, Succ<M>, Succ<N>>
where
    T: Copy,
    M: Nat + Val,
    N: Nat + Val,
    Succ<M>: Nat + Val,
    Succ<N>: Nat + Val,
{
    /// Get the submatrix at index (i, j)
    pub fn submatrix(&self, i: usize, j: usize) -> Matrix<T, M, N> {
        self.remove_row(i).remove_col(j)
    }
}

/// Yes this should be N-Dim (the N3 and N4 implementations are in fact that) but
/// Rust won't accept my inductive definition and after wasting two hours this is
/// just became a trait and got seperate implementations for the sizes I need.
pub trait Determinant<T, M> {
    /// Calculate the minor(determinant of the submatrix) of a MxM matrix at (i,j)
    fn minor(&self, i: usize, j: usize) -> T;
    /// Calculate the cofactor of a MxM matrix at (i, j)
    fn cofactor(&self, i: usize, j: usize) -> T;
    /// Calculate the determinant of the matrix
    fn det(&self) -> T;
}

impl<T: Num + Copy> Determinant<T, N2> for Matrix<T, N2, N2> {
    fn minor(&self, _i: usize, _j: usize) -> T {
        unimplemented!()
    }
    fn cofactor(&self, _i: usize, _j: usize) -> T {
        unimplemented!()
    }
    fn det(&self) -> T {
        match self.data[..] {
            [a, b, c, d] => a * d - b * c,
            _ => unreachable!(),
        }
    }
}

impl<T: Num + Copy + Signed + std::iter::Sum> Determinant<T, N3> for Matrix<T, N3, N3> {
    fn minor(&self, i: usize, j: usize) -> T {
        self.submatrix(i, j).det()
    }
    fn cofactor(&self, i: usize, j: usize) -> T {
        if (i + j).is_odd() {
            -T::one() * self.minor(i, j)
        } else {
            self.minor(i, j)
        }
    }
    fn det(&self) -> T {
        match self.data[..] {
            [a, b, c, d] => a * d - b * c,
            _ => (0..self.height())
                .map(|j| self[(0, j)] * self.cofactor(0, j))
                .sum(),
        }
    }
}

impl<T: Num + Copy + Signed + std::iter::Sum> Determinant<T, N4> for Matrix<T, N4, N4> {
    /// Calculate the minor(determinant of the submatrix) of a 4x4 matrix at (i,j)
    fn minor(&self, i: usize, j: usize) -> T {
        self.submatrix(i, j).det()
    }
    /// Calculate the cofactor of a 4x4 matrix at (i, j)
    fn cofactor(&self, i: usize, j: usize) -> T {
        if (i + j).is_odd() {
            -T::one() * self.minor(i, j)
        } else {
            self.minor(i, j)
        }
    }
    fn det(&self) -> T {
        match self.data[..] {
            [a, b, c, d] => a * d - b * c,
            _ => (0..self.height())
                .map(|j| self[(0, j)] * self.cofactor(0, j))
                .sum(),
        }
    }
}

impl<T: Num + Copy + Signed + std::iter::Sum + NumAssignOps> Matrix<T, N4, N4> {
    /// Calculate the inverse of a matrix if it exists - return None otherwise
    pub fn invert(&self) -> Option<Self> {
        let det = self.det();
        if det == T::zero() {
            None
        } else {
            let mut m = Self::new_uninitialized();
            m.data = self
                .iter_indexed()
                .map(|(i, j, _)| self.cofactor(i, j))
                .collect::<Vec<_>>();
            let mut m2 = m.transpose();
            let _ = m2.iter_mut().map(|x| *x /= det).collect::<Vec<_>>();
            Some(m2)
        }
    }
}

/// Convert an 1x1 Matrix to a scalar
impl<T: Copy> Matrix<T, N1, N1> {
    pub fn as_scalar(&self) -> T {
        self[(0, 0)]
    }
}
