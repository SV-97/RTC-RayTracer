//! "Mathy" methods and trait implementations / General matrix arithmetic

use num_traits::{Float, Num, One, Signed, Zero};

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use crate::utils::typelevel_nums::*;

use super::prelude::*;

/// A + B
impl<T, M, N> Add for Matrix<T, M, N>
where
    T: Num,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Matrix::from(
            self.into_iter()
                .zip(other.into_iter())
                .map(|(l, r)| l + r)
                .collect::<Vec<_>>(),
        )
    }
}

/// A + &B
impl<T, M, N> Add<&Self> for Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Self;
    fn add(self, other: &Self) -> Self::Output {
        Matrix::from(
            self.into_iter()
                .zip(other.iter())
                .map(|(l, r)| l + *r)
                .collect::<Vec<_>>(),
        )
    }
}

/// &A + B
impl<T, M, N> Add<Matrix<T, M, N>> for &Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Matrix<T, M, N>;
    fn add(self, other: Matrix<T, M, N>) -> Self::Output {
        Matrix::from(
            self.iter()
                .zip(other.into_iter())
                .map(|(l, r)| *l + r)
                .collect::<Vec<_>>(),
        )
    }
}

/// &A + &B
impl<T, M, N> Add for &Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Matrix<T, M, N>;
    fn add(self, other: Self) -> Self::Output {
        Matrix::from(
            self.iter()
                .zip(other.iter())
                .map(|(l, r)| *l + *r)
                .collect::<Vec<_>>(),
        )
    }
}

/// A - B
impl<T, M, N> Sub for Matrix<T, M, N>
where
    T: Num,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Matrix::from(
            self.into_iter()
                .zip(other.into_iter())
                .map(|(l, r)| l - r)
                .collect::<Vec<_>>(),
        )
    }
}

/// A - &B
impl<T, M, N> Sub<&Self> for Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Self;
    fn sub(self, other: &Self) -> Self::Output {
        Matrix::from(
            self.into_iter()
                .zip(other.iter())
                .map(|(l, r)| l - *r)
                .collect::<Vec<_>>(),
        )
    }
}

/// &A + B
impl<T, M, N> Sub<Matrix<T, M, N>> for &Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Matrix<T, M, N>;
    fn sub(self, other: Matrix<T, M, N>) -> Self::Output {
        Matrix::from(
            self.iter()
                .zip(other.into_iter())
                .map(|(l, r)| *l - r)
                .collect::<Vec<_>>(),
        )
    }
}

/// &A - &B
impl<T, M, N> Sub for &Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Matrix<T, M, N>;
    fn sub(self, other: Self) -> Self::Output {
        Matrix::from(
            self.iter()
                .zip(other.iter())
                .map(|(l, r)| *l - *r)
                .collect::<Vec<_>>(),
        )
    }
}

/// A += B
impl<T, M, N> AddAssign for Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    fn add_assign(&mut self, other: Self) {
        *self = Matrix::from(
            self.iter()
                .zip(other.into_iter())
                .map(|(l, r)| *l + r)
                .collect::<Vec<_>>(),
        )
    }
}

/// A += &B
impl<T, M, N> AddAssign<&Self> for Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    fn add_assign(&mut self, other: &Self) {
        *self = Matrix::from(
            self.iter()
                .zip(other.iter())
                .map(|(l, r)| *l + *r)
                .collect::<Vec<_>>(),
        )
    }
}

/// -A
impl<T, M, N> Neg for Matrix<T, M, N>
where
    T: Signed,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Matrix::from(self.into_iter().map(Neg::neg).collect::<Vec<_>>())
    }
}

/// -&A
impl<T, M, N> Neg for &Matrix<T, M, N>
where
    T: Signed + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Matrix<T, M, N>;
    fn neg(self) -> Self::Output {
        Matrix::from(self.iter().map(|x| Neg::neg(*x)).collect::<Vec<_>>())
    }
}

/// A * b
impl<T, M, N> Mul<T> for Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Matrix::from(self.into_iter().map(|x| x * other).collect::<Vec<_>>())
    }
}

/// &A * b
impl<T, M, N> Mul<T> for &Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Matrix<T, M, N>;
    fn mul(self, other: T) -> Self::Output {
        Matrix::from(self.iter().map(|x| *x * other).collect::<Vec<_>>())
    }
}

/// A / b
impl<T, M, N> Div<T> for Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Matrix::from(self.into_iter().map(|x| x / other).collect::<Vec<_>>())
    }
}

/// &A / b
impl<T, M, N> Div<T> for &Matrix<T, M, N>
where
    T: Num + Copy,
    M: Nat + Val,
    N: Nat + Val,
{
    type Output = Matrix<T, M, N>;
    fn div(self, other: T) -> Self::Output {
        Matrix::from(self.iter().map(|x| *x / other).collect::<Vec<_>>())
    }
}

/// Multiply a matrix A with another matrix B
/// A * B
impl<T, MA, N, NB> Mul<Matrix<T, N, NB>> for Matrix<T, MA, N>
where
    T: Num + Default + Copy + std::iter::Sum<T>,
    MA: Nat + Val,
    N: Nat + Val,
    NB: Nat + Val,
{
    type Output = Matrix<T, MA, NB>;

    fn mul(self, other: Matrix<T, N, NB>) -> Self::Output {
        let mut new = Matrix::new();
        for (i, row) in self.iter_rows().enumerate() {
            let row = row.collect::<Vec<_>>();
            for (j, col) in other.iter_cols().enumerate() {
                new[(i, j)] = row.iter().zip(col).map(|(r, c)| **r * *c).sum();
            }
        }
        new
    }
}

/// Multiply a matrix A with another matrix B
/// &A * B
impl<T, MA, N, NB> Mul<Matrix<T, N, NB>> for &Matrix<T, MA, N>
where
    T: Num + Default + Copy + std::iter::Sum<T>,
    MA: Nat + Val,
    N: Nat + Val,
    NB: Nat + Val,
{
    type Output = Matrix<T, MA, NB>;

    fn mul(self, other: Matrix<T, N, NB>) -> Self::Output {
        let mut new = Matrix::new();
        for (i, row) in self.iter_rows().enumerate() {
            let row = row.collect::<Vec<_>>();
            for (j, col) in other.iter_cols().enumerate() {
                new[(i, j)] = row.iter().zip(col).map(|(r, c)| **r * *c).sum();
            }
        }
        new
    }
}

/// Multiply a matrix A with another matrix B
/// A * &B
impl<T, MA, N, NB> Mul<&Matrix<T, N, NB>> for Matrix<T, MA, N>
where
    T: Num + Default + Copy + std::iter::Sum<T>,
    MA: Nat + Val,
    N: Nat + Val,
    NB: Nat + Val,
{
    type Output = Matrix<T, MA, NB>;

    fn mul(self, other: &Matrix<T, N, NB>) -> Self::Output {
        let mut new = Matrix::new();
        for (i, row) in self.iter_rows().enumerate() {
            let row = row.collect::<Vec<_>>();
            for (j, col) in other.iter_cols().enumerate() {
                new[(i, j)] = row.iter().zip(col).map(|(r, c)| **r * *c).sum();
            }
        }
        new
    }
}

/// Multiply a matrix A with another matrix B
/// &A * &B
impl<T, MA, N, NB> Mul<&Matrix<T, N, NB>> for &Matrix<T, MA, N>
where
    T: Num + Default + Copy + std::iter::Sum<T>,
    MA: Nat + Val,
    N: Nat + Val,
    NB: Nat + Val,
{
    type Output = Matrix<T, MA, NB>;

    fn mul(self, other: &Matrix<T, N, NB>) -> Self::Output {
        let mut new = Matrix::new();
        for (i, row) in self.iter_rows().enumerate() {
            let row = row.collect::<Vec<_>>();
            for (j, col) in other.iter_cols().enumerate() {
                new[(i, j)] = row.iter().zip(col).map(|(r, c)| **r * *c).sum();
            }
        }
        new
    }
}

impl<T, M, N> Matrix<T, M, N>
where
    T: Signed,
    M: Nat + Val,
    N: Nat + Val,
{
    /// Take absolute value of each element of the matrix
    fn abs(self) -> Self {
        Matrix::from(self.iter().map(T::abs).collect::<Vec<_>>())
    }
}

impl<T, M> Matrix<T, M, N1>
where
    T: Float + Copy + std::iter::Sum,
    M: Nat + Val,
{
    /// Compute the magnitude/norm/length of the vector
    pub fn mag(&self) -> T {
        let s: T = self.iter().map(|x| *x * *x).sum();
        s.sqrt()
    }

    /// Normalize a vector such that you get a vector with same direction
    /// but a magnitude of 1.
    pub fn unit(self) -> Self {
        let mag = self.mag();
        self / mag
    }
}

impl<T: Copy, M: Nat + Val, N: Nat + Val> Matrix<T, M, N> {
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut v = Vec::with_capacity(M::val() * N::val());
        for i in 0..M::val() {
            for j in 0..N::val() {
                v.push(self[(j, i)]);
            }
        }
        Matrix::from(v)
    }
}

impl<T, M> Matrix<T, M, M>
where
    T: Clone + One + Zero,
    M: Nat + Val,
{
    pub fn identity() -> Self {
        let mut m = Matrix::new_uninitialized();
        m.data = vec![T::zero(); M::val() * M::val()];
        for i in 0..M::val() {
            m[(i, i)] = T::one();
        }
        m
    }
}
