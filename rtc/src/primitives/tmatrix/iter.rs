//! Iteration and indexing operations and methods

use std::ops::{Index, IndexMut};

use crate::utils::typelevel_nums::*;

use super::prelude::*;
// Indexing
impl<T, M: Nat + Val, N: Nat + Val> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        let (i, j) = coords;
        &self.data[Self::to_row_major(i, j)]
    }
}

impl<T, M: Nat + Val, N: Nat + Val> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        let (i, j) = coords;
        &mut self.data[Self::to_row_major(i, j)]
    }
}

// General iteration
impl<T, M: Nat + Val, N: Nat + Val> Matrix<T, M, N> {
    /// Convert a pair of matrix coordinates to an index into a data buffer
    /// that's implemented in row_major order
    /// It's interpreted as i'th row and j'th column
    #[allow(clippy::wrong_self_convention)]
    pub fn to_row_major(i: usize, j: usize) -> usize {
        N::val() * i + j
    }

    /// Iterate over all elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Iterate over all elements mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
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

    /// Iterate over 3-tuples (i, j, self[(i,j)])
    pub fn iter_indexed(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.iter_rows()
            .enumerate()
            .map(|(i, iter)| iter.enumerate().map(move |(j, x)| (i, j, x)))
            .flatten()
    }

    /// Iterate over all elements deinitializing self
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter()
    }

    /// Iterate over the ith row of the matrix deinitializing self
    pub fn into_iter_row(self, i: usize) -> impl Iterator<Item = T> {
        self.into_iter().skip(i * N::val()).take(N::val())
    }
}

// Consuming iteration
impl<T, M: Nat + Val, N: Nat + Val> Matrix<T, M, N>
where
    Matrix<T, M, N>: Clone,
{
    /// Iterate over the jth coloumn of the matrix deinitializing self
    pub fn into_iter_col(self, j: usize) -> impl Iterator<Item = T> {
        self.into_iter().skip(j).step_by(N::val())
    }

    /// Iterate over all rows of the matrix deinitializing self
    // FIXME Find more performant solution over clone
    pub fn into_iter_rows(self) -> impl Iterator<Item = impl Iterator<Item = T>> {
        (0..M::val()).map(move |i| self.clone().into_iter_row(i))
    }

    /// Iterate over all rows of the matrix deinitializing self
    // FIXME Find more performant solution over clone
    pub fn into_iter_cols(self) -> impl Iterator<Item = impl Iterator<Item = T>> {
        (0..N::val()).map(move |j| self.clone().into_iter_col(j))
    }

    /// Iterate over 3-tuples (i, j, self[(i,j)]) deinitializing self
    pub fn into_iter_indexed(self) -> impl Iterator<Item = (usize, usize, T)> {
        self.into_iter_rows()
            .enumerate()
            .map(|(i, iter)| iter.enumerate().map(move |(j, x)| (i, j, x)))
            .flatten()
    }
}
