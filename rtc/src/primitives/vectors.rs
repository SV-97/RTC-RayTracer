use num_traits::Num;

use crate::utils::typelevel_nums::*;

use super::tmatrix::Matrix;

type Vec4D<T> = Matrix<T, N4, N1>;


#[macro_export]
macro_rules! vec4 {
    ( $x:expr, $y:expr, $z:expr, $w:expr ) => {{
        Vec4D::from(vec![$x, $y, $z, $w])
    }};
}

/// Construct a new vector in 3D space
pub fn vec3<T: Num> (x: T, y: T, z: T) -> Vec4D<T> {
    vec4![x, y, z, T::zero()]
}

/// Construct a new point in 3D space
pub fn point<T: Num> (x: T, y: T, z: T) -> Vec4D<T> {
    vec4![x, y, z, T::one()]
}

impl<T: Copy> Vec4D<T> {
    pub fn x(&self) -> T {
        self[(0,0)]
    }

    pub fn y(&self) -> T {
        self[(1,0)]
    }

    pub fn z(&self) -> T {
        self[(2,0)]
    }

    pub fn w(&self) -> T {
        self[(3,0)]
    }
}

impl<T: Num + Copy + Default + std::iter::Sum<T>> Vec4D<T> {

    /// Calculate the scalar product of two vectors
    pub fn scalar_prod(self, other: Self) -> T {
        let m = self.transpose() * other;
        m.as_scalar()
    }

    /// Cross product between two vectors
    pub fn cross(self, other: Self) -> Self {
       vec3(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
       )
    }

    /// x direction unit vector
    pub fn unit_x() -> Self {
        vec3(T::one(), T::zero(), T::zero())
    }
    /// y direction unit vector
    pub fn unit_y() -> Self {
        vec3(T::zero(), T::one(), T::zero())
    }

    /// z direction unit vector
    pub fn unit_z() -> Self {
        vec3(T::zero(), T::zero(), T::one())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::approx_eq::ApproxEq;

    #[test]
    fn scalar_prod() {
        let v1 = vec3(1.0, -2.0, 3.0);
        let v2 = vec3(5.0, 8.2, -3.1);
        assert!(v1.scalar_prod(v2).approx_eq(5.0 - 16.4 - 9.3));
    }
}
