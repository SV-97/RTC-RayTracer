use num::Integer;
use num_traits::{Num, NumAssignOps, One, Signed, Zero};

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

    /// Iterate over all elements mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn iter_indexed(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.iter_rows()
            .enumerate()
            .map(|(i, iter)| iter.enumerate().map(move |(j, x)| (i, j, x)))
            .flatten()
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

impl<T: Copy, M: Nat + Val, N: Nat + Val> Matrix<T, M, N> {
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut m = Matrix::new_uninitialized();
        let mut v = Vec::with_capacity(M::val() * N::val());
        for i in 0..M::val() {
            for j in 0..N::val() {
                v.push(self[(j, i)]);
            }
        }
        m.data = v;
        m
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
    pub fn submatrix(&self, i: usize, j: usize) -> Matrix<T, M, N> {
        self.remove_row(i).remove_col(j)
    }
}

/// Yes this should be N-Dim (the N3 and N4 implementations are in fact that) but
/// Rust won't accept my inductive definition and after wasting two hours this is
/// just became a trait and got seperate implementations for the sizes I need.
trait Determinant<T, M> {
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
    fn invert(&self) -> Option<Self> {
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

        let b = matrix![ N3, N3 =>
            1, 2, 6;
            -5, 8, -4;
            2, 6, 4
        ];
        assert_eq!(b.cofactor(0, 0), 56);
        assert_eq!(b.cofactor(0, 1), 12);
        assert_eq!(b.cofactor(0, 2), -46);
        assert_eq!(b.det(), -196);

        let c = matrix![ N4, N4 =>
            -2, -8, 3, 5;
            -3, 1, 7, 3;
            1, 2, -9, 6;
            -6, 7, 7, -9
        ];
        assert_eq!(c.cofactor(0, 0), 690);
        assert_eq!(c.cofactor(0, 1), 447);
        assert_eq!(c.cofactor(0, 2), 210);
        assert_eq!(c.cofactor(0, 3), 51);
        assert_eq!(c.det(), -4071);
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

    #[test]
    fn minor() {
        let a = matrix![ N3, N3 =>
            3, 5, 0;
            2, -1, -7;
            6, -1, 5
        ];
        let b = a.submatrix(1, 0);
        assert_eq!(a.minor(1, 0), 25);
        assert_eq!(a.minor(1, 0), b.det());
    }

    #[test]
    fn invert() {
        let a = matrix![ N4, N4 =>
            6, 4, 4, 4;
            5, 5, 7, 6;
            4, -9, 3, -7;
            9, 1, 7, -6
        ];
        assert_eq!(a.det(), -2120);
        assert!(a.invert().is_some());

        let b = matrix![ N4, N4 =>
            -4.,  2., -2., -3.;
             9.,  6.,  2.,  6.;
             0., -5.,  1., -5.;
             0.,  0.,  0.,  0.
        ];
        assert_eq!(b.det(), 0.0);
        assert!(b.invert().is_none());

        let c = matrix![ N4, N4 =>
            -5.,  2.,  6., -8.;
             1., -5.,  1.,  8.;
             7.,  7., -6., -7.;
             1., -3.,  7.,  4.
        ];
        let c_inv = matrix![ N4, N4 =>
             0.21805,  0.45113,  0.24060, -0.04511;
            -0.80827, -1.45677, -0.44361,  0.52068;
            -0.07895, -0.22368, -0.05263,  0.19737;
            -0.52256, -0.81391, -0.30075,  0.30639
        ];
        assert!(c.det().approx_eq(532.0));
        assert!(c.cofactor(2, 3).approx_eq(-160.0));
        dbg!(c_inv[(3, 2)], -160.0 / 532.0);
        assert!(c_inv[(3, 2)].approx_eq(-160.0 / 532.0));
        assert!(c.cofactor(3, 2).approx_eq(105.0));
        assert!(c_inv[(2, 3)].approx_eq(105.0 / 532.0));
        assert!(c.invert().unwrap().approx_eq(&c_inv));

        let d = matrix![ N4, N4 =>
             8., -5.,  9.,  2.;
             7.,  5.,  6.,  1.;
            -6.,  0.,  9.,  6.;
            -3.,  0., -9., -4.
        ];
        assert!(d.invert().unwrap().approx_eq(&matrix![ N4, N4 =>
            -0.15385, -0.15385, -0.28205, -0.53846;
            -0.07692,  0.12308,  0.02564,  0.03077;
             0.35897,  0.35897,  0.43590,  0.92308;
            -0.69231, -0.69231, -0.76923, -1.92308
        ]));

        let e = matrix![ N4, N4 =>
             9.,  3.,  0.,  9.;
            -5., -2., -6., -3.;
            -4.,  9.,  6.,  4.;
            -7.,  6.,  6.,  2.
        ];
        assert!(e.invert().unwrap().approx_eq(&matrix![ N4, N4 =>
            -0.04074, -0.07778,  0.14444, -0.22222;
            -0.07778,  0.03333,  0.36667, -0.33333;
            -0.02901, -0.14630, -0.10926,  0.12963;
             0.17778,  0.06667, -0.26667,  0.33333
        ]));
    }
}
