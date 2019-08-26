use num_traits::{Num, One, Zero};

use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Mul};

use crate::utils::typelevel_nums::*;

use super::approx_eq::ApproxEq;

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
            {type M = $m;
            type N = $n;
            use $crate::primitives::tmatrix::Matrix;
            let data = vec![ $( vec![$($val),+] ),* ];
            assert_eq!(data.len(), M::val());
            assert_eq!(data[0].len(), N::val());
            let flattened = data.into_iter().map(|arr| arr.into_iter()).flatten().collect::<Vec<_>>();
            assert_eq!(flattened.len(), M::val() * N::val());
            let mut m = Matrix::<_, M, N>::new_uninitialized();
            m.data = flattened;
            let m = m;
            m}
        }
    };
    ( $m:ty, $n:ty => $( $( $val:expr )+ );* ) => {
        {
            {type M = $m;
            type N = $n;
            use $crate::primitives::tmatrix::Matrix;
            let data = vec![ $( vec![$($val),+] ),* ];
            assert_eq!(data.len(), M::val());
            assert_eq!(data[0].len(), N::val());
            let flattened = data.into_iter().map(|arr| arr.into_iter()).flatten().collect::<Vec<_>>();
            assert_eq!(flattened.len(), M::val() * N::val());
            let mut m = Matrix::<_, M, N>::new_uninitialized();
            m.data = flattened;
            let m = m;
            m}
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

    pub fn width(&self) -> usize {
        N::val()
    }

    /// Convert a pair of matrix coordinates to an index into a data buffer
    /// that's implemented in row_major order
    /// It's interpreted as i'th row and j'th column
    pub fn to_row_major(i: usize, j: usize) -> usize {
        N::val() * i + j
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

impl<T, M: Nat + Val, N: Nat + Val> fmt::Debug for Matrix<T, M, N>
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

impl<T: ApproxEq + Copy, M: Nat + Val, N: Nat + Val> ApproxEq<T> for &Matrix<T, M, N> {
    const EPSILON: T = T::EPSILON;
    fn approx_eq(self, other: Self) -> bool {
        self.iter().zip(other.iter()).all(|(l, r)| l.approx_eq(*r))
    }
}

/// Multiply a matrix A with another matrix B
impl<T, MA, N, NB> Mul<Matrix<T, N, NB>> for Matrix<T, MA, N>
where
    T: Num + Default + Copy + std::iter::Sum<T> + fmt::Debug,
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
                new[(i, j)] = row
                    .iter()
                    .zip(col)
                    .map(|(r, c)| {
                        println!("{:?}*{:?}", r, c);
                        **r * *c
                    })
                    .sum();
            }
        }
        new
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
    /// Remove the i'th row
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
    pub fn submatrix(&self, i: usize, j: usize) -> Matrix<T, M, N> {
        self.remove_row(i).remove_col(j)
    }
}

impl<T: Num + Copy> Matrix<T, N2, N2> {
    /// Calculate determinant of a 2x2 matrix.
    pub fn det(&self) -> T {
        match self.data[..] {
            [a, b, c, d] => a * d - b * c,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let a = matrix![ N2, N3 =>
            1 2 3;
            5 6 7
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

    #[test]
    fn iter_col() {
        let a = matrix![ N4, N3 =>
            1 2 3;
            5 6 7;
            9 8 7;
            6 5 4
        ];
        assert_eq!(
            a.iter_col(0).map(|x| *x).collect::<Vec<_>>(),
            vec![1, 5, 9, 6]
        );
        assert_eq!(
            a.iter_col(1).map(|x| *x).collect::<Vec<_>>(),
            vec![2, 6, 8, 5]
        );
        assert_eq!(
            a.iter_col(2).map(|x| *x).collect::<Vec<_>>(),
            vec![3, 7, 7, 4]
        );
    }

    #[test]
    fn iter_row() {
        let a = matrix![ N3, N4 =>
            1 5 9 6;
            2 6 8 5;
            3 7 7 4
        ];
        assert_eq!(
            a.iter_row(0).map(|x| *x).collect::<Vec<_>>(),
            vec![1, 5, 9, 6]
        );
        assert_eq!(
            a.iter_row(1).map(|x| *x).collect::<Vec<_>>(),
            vec![2, 6, 8, 5]
        );
        assert_eq!(
            a.iter_row(2).map(|x| *x).collect::<Vec<_>>(),
            vec![3, 7, 7, 4]
        );
    }

    #[test]
    fn iter_rows() {
        let a = matrix![ N4, N3 =>
            1 2 3;
            5 6 7;
            9 8 7;
            6 5 4
        ];
        let mut b = a.iter_rows();
        assert_eq!(
            b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert_eq!(
            b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
            vec![5, 6, 7]
        );
        assert_eq!(
            b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
            vec![9, 8, 7]
        );
        assert_eq!(
            b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
            vec![6, 5, 4]
        );
    }

    #[test]
    fn iter_cols() {
        let a = matrix![ N4, N3 =>
            1 2 3;
            5 6 7;
            9 8 7;
            6 5 4
        ];
        let mut b = a.iter_cols();
        assert_eq!(
            b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
            vec![1, 5, 9, 6]
        );
        assert_eq!(
            b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
            vec![2, 6, 8, 5]
        );
        assert_eq!(
            b.next().unwrap().map(|x| *x).collect::<Vec<_>>(),
            vec![3, 7, 7, 4]
        );
    }

    #[test]
    fn index() {
        let a = matrix![ N6, N3 =>
            1, 2, 3;
            5, 6, 7;
            9, 8, 7;
            6, 5, 4;
            3, 2, 1;
            0, -1, -2
        ];
        assert_eq!(a[(2, 0)], 9);
    }

    #[test]
    fn mul() {
        let a = matrix![ N3, N4 =>
            1 2 3 4;
            5 6 7 8;
            9 8 7 6
        ];
        let b = matrix![ N4, N4 =>
            -2, 1, 2, 3;
            3, 2, 1, -1;
            4, 3, 6, 5;
            1, 2, 7, 8
        ];
        let c = matrix![ N3, N4 =>
            20 22 50 48;
            44 54 114 108;
            40 58 110 102
        ];
        assert_eq!(a * b, c);
    }

    #[test]
    fn identity() {
        let a = matrix![ N3, N3 =>
            1 2 3;
            4 5 6;
            7 8 9
        ];
        assert_eq!(a.clone() * <Matrix<_, N3, N3>>::identity(), a.clone());
    }

    #[test]
    fn det() {
        let a = matrix![ N2, N2 =>
            1 5 ;
            {-3} 2
        ];
        assert_eq!(a.det(), 17);
    }

    #[test]
    fn remove_row() {
        let a = matrix![ N5, N5 =>
            1 5 3 4 5;
            2 2 7 9 8;
            3 2 1 7 9;
            5 5 3 6 1;
            0 5 0 2 4
        ];
        assert_eq!(
            a.remove_row(0),
            matrix![ N4, N5 =>
                2 2 7 9 8;
                3 2 1 7 9;
                5 5 3 6 1;
                0 5 0 2 4
            ]
        );
    }

    #[test]
    fn remove_col() {
        let a = matrix![ N5, N5 =>
            1 5 3 4 5;
            2 2 7 9 8;
            3 2 1 7 9;
            5 5 3 6 1;
            0 5 0 2 4
        ];
        assert_eq!(
            a.remove_col(2),
            matrix![ N5, N4 =>
                1 5 4 5;
                2 2 9 8;
                3 2 7 9;
                5 5 6 1;
                0 5 2 4
            ]
        );
    }

    #[test]
    fn submatrix() {
        let a = matrix![ N3, N3 =>
            1, 5, 0;
            -3, 2, 2;
            0, 6, -3
        ];
        assert_eq!(
            a.submatrix(0, 2),
            matrix![ N2, N2 =>
                -3, 2;
                0, 6
            ]
        );
        let b = matrix![ N4, N4 =>
            -6, 1, 1, 6;
            -8, 5, 8, 6;
            -1, 0, 8, 2;
            -7, 1, -1, 1
        ];
        assert_eq!(
            b.submatrix(2, 1),
            matrix![ N3, N3 =>
                -6, 1, 6;
                -8, 8, 6;
                -7, -1, 1
            ]
        );
    }
}
