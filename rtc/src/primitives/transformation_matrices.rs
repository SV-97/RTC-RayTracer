use num_traits::Num;

use crate::utils::typelevel_nums::*;

use super::tmatrix::Matrix;


impl<T: Num + Copy> Matrix<T, N4, N4> {
    pub fn translate(x: T, y: T, z: T) -> Self {
        matrix![ N4, N4 =>
            T::zero() T::zero() T::zero() x;
            T::zero() T::one()  T::zero() y;
            T::zero() T::zero() T::one()  z;
            T::zero() T::zero() T::zero() T::one()
        ]
    }
}