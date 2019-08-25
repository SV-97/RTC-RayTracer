#![feature(const_generics)]

use num_traits::Num;

use std::fmt;
use std::ops::{Index, IndexMut, Mul};

use super::approx_eq::ApproxEq;

// This should really use const generics, but they're not stable
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}", self.width, self.height)?;
        for row in self.iter_rows() {
            writeln!(f, "    {:?}", row.collect::<Vec<_>>())?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! matrix {
    ( $( $( $val:expr ),+ );* ) => {
        {
            use $crate::primitives::matrix::Matrix;
            let data = [ $( [ $($val),+ ] ),* ];
            let height = data.len();
            let width = data[0].len();
            let mut m = Matrix::new(width, height);
            m.data = data.into_iter().flatten().map(|x| *x).collect::<Vec<_>>();
            let m = m;
            m
        }
    }
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            width,
            height,
            data: vec![Default::default(); width * height],
        }
    }
}

impl<T> Matrix<T> {
    /// Convert a pair of coordinates matrix coordinates to an index into the data buffer
    pub fn as_one_dim(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    /// Set a single field in the matrix. Indices are checked.
    pub fn put(&mut self, x: usize, y: usize, data: T) -> Result<(), String> {
        if x < self.width {
            if y < self.height {
                let i = self.as_one_dim(x, y);
                self.data[i] = data;
                Ok(())
            } else {
                Err(format!(
                    "Tried accessing canvas out of bounds. Max y-index={}, actual index={}.",
                    self.height - 1,
                    y
                ))
            }
        } else {
            Err(format!(
                "Tried accessing canvas out of bounds. Max x-index={}, actual index={}.",
                self.width - 1,
                x
            ))
        }
    }

    /// Read data at the given coordinates. Indices are checked
    pub fn get(&self, x: usize, y: usize) -> Result<&T, String> {
        if x < self.width {
            if y < self.height {
                let i = self.as_one_dim(x, y);
                Ok(&self.data[i])
            } else {
                Err(format!(
                    "Tried accessing canvas out of bounds. Max y-index={}, actual index={}.",
                    self.height - 1,
                    y
                ))
            }
        } else {
            Err(format!(
                "Tried accessing canvas out of bounds. Max x-index={}, actual index={}.",
                self.width - 1,
                x
            ))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Iterate over all the rows of the canvas
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        let mut v = vec![];
        for i in 0..self.height {
            v.push(self.iter().skip(i * self.width).take(self.width));
        }
        v.into_iter()
    }

    /// Iterate over all the columns of the canvas
    pub fn iter_columns(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        let mut v = vec![];
        for i in 0..self.width {
            v.push(self.iter().skip(i).step_by(self.width));
        }
        v.into_iter()
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        let (x, y) = coords;
        &self.data[self.as_one_dim(x, y)]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        let (x, y) = coords;
        let i = self.as_one_dim(x, y);
        &mut self.data[i]
    }
}

impl<T: ApproxEq + Copy> ApproxEq<T> for &Matrix<T> {
    const EPSILON: T = T::EPSILON;
    fn approx_eq(self, other: Self) -> bool {
        self.iter().zip(other.iter()).all(|(l, r)| l.approx_eq(*r))
    }
}

impl<T: Num + Default + Copy + std::iter::Sum<T>> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        if self.width == other.height {
            let mut new = Matrix::new(self.height, other.width);
            for (i, row) in self.iter_rows().enumerate() {
                let row = row.collect::<Vec<_>>();
                for (j, col) in other.iter_columns().enumerate() {
                    new[(j, i)] = row.iter().zip(col).map(|(r, c)| **r * *c).sum();
                }
            }
            Some(new)
        } else {
            None
        }
    }
}

/// NEXT UP SHOULD BE THE IDENTITY MATRIX

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;

    #[test]
    fn read_and_write() {
        let mut c = Matrix::new(2, 2);
        c[(0, 0)] = 3;
        c[(1, 0)] = 4;
        c[(1, 1)] = 5;
        assert_eq!(c.data[0], 3);
        assert_eq!(c.data[1], 4);
        assert_eq!(c.data[2], Default::default());
        assert_eq!(c.data[3], 5);
    }

    #[test]
    fn iter_rows() {
        let mut c = Matrix::new(2, 2);
        c[(0, 0)] = 3;
        c[(1, 0)] = 4;
        c[(1, 1)] = 5;
        let mut i = c.iter_rows();
        for p in c.iter_rows() {
            for p in p {
                dbg!(p);
            }
        }
        let mut row1 = i.next().unwrap();
        let mut row2 = i.next().unwrap();
        assert!(i.next().is_none());
        assert_eq!(row1.next().unwrap(), &3);
        assert_eq!(row1.next().unwrap(), &4);
        assert_eq!(row2.next().unwrap(), &usize::default());
        assert_eq!(row2.next().unwrap(), &5);
    }

    #[test]
    fn mul() {
        let a = matrix![
            1_isize, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2
        ];
        let b = matrix![
            -2, 1, 2, 3;
            3, 2, 1, -1;
            4, 3, 6, 5;
            1, 2, 7, 8
        ];
        let c = matrix![
            20, 22, 50, 48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26, 46, 42
        ];
        assert_eq!(a * b, Some(c));
    }
}
