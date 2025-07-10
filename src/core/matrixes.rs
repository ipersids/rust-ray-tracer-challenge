//! # Matrix<N> Module
//!
//! A generic square matrix of size NxN.
//! This matrix implementation provides common linear algebra operations
//! such as matrix multiplication, determinant calculation, and transformations.

use crate::core::Tuple;
use crate::core::approx_eq;
use std::convert::From;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Matrix<const N: usize> {
    content: [[f64; N]; N],
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Matrix<N> {
    /// Creates a new matrix with all elements initialized to 0.0.
    pub fn new() -> Self {
        Matrix {
            content: [[0.0; N]; N],
        }
    }

    /// Returns the dimension of the square matrix.
    pub fn size(&self) -> usize {
        N
    }

    /// Safely retrieves a value at the specified position with bounds checking.
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if col >= N || row >= N {
            return None;
        }
        Some(self[row][col])
    }

    /// Creates an identity matrix (1.0 on the diagonal, 0.0 elsewhere).
    pub fn identity() -> Self {
        let mut res = Matrix::<N>::new();

        for row in 0..N {
            for col in 0..N {
                if row == col {
                    res[row][col] = 1.0;
                } else {
                    res[row][col] = 0.0;
                }
            }
        }

        res
    }
}

impl<const N: usize> Matrix<N> {
    /// Creates a new matrix that is the transpose of the original.
    pub fn transpose(&self) -> Self {
        let mut res = Matrix::<N>::new();

        for row in 0..N {
            for col in 0..N {
                res[col][row] = self[row][col];
            }
        }

        res
    }

    /// Tests a matrix for invertibility.
    pub fn is_invertible(&self) -> bool {
        !approx_eq(self.determinant(), 0.0)
    }

    /// Produces the inverse of the given matrix.
    pub fn inverse(&self) -> Result<Self, &'static str> {
        let det: f64 = self.determinant();
        if approx_eq(det, 0.0) {
            return Err("Matrix is not invertible");
        }

        let mut res: Matrix<N> = Matrix::<N>::new();
        for row in 0..N {
            for col in 0..N {
                res[col][row] = self.cofactor(row, col) / det;
            }
        }

        Ok(res)
    }
}

// @note how to restrict N to be (N <= 4 and N > 0) witout rintime assert cost ?
impl<const N: usize> Matrix<N> {
    /// Creates a submatrix by removing the specified row and column.
    pub fn submatrix<const M: usize>(&self, rm_row: usize, rm_col: usize) -> Matrix<M>
    where
        [(); M]: Sized,
    {
        assert_eq!(N - 1, M, "Invalid size of {} (expected {})", M, N - 1);

        let mut res = Matrix::<M>::new();

        let mut row: usize = 0;
        for src_row in 0..N {
            if src_row == rm_row {
                continue;
            }

            let mut col = 0;
            for src_col in 0..N {
                if src_col == rm_col {
                    continue;
                }
                res[row][col] = self[src_row][src_col];
                col += 1;
            }

            row += 1;
        }
        res
    }

    /// Calculates the determinant of the matrix.
    ///
    /// The implementation uses cofactor expansion for matrices larger than 2x2.
    /// Support matrices up to 4x4 size.
    pub fn determinant(&self) -> f64 {
        if self.size() == 1 {
            return self[0][0];
        }
        if self.size() == 2 {
            return self[0][0] * self[1][1] - self[1][0] * self[0][1];
        }

        let mut det: f64 = 0.0;
        for col in 0..N {
            det += self[0][col] * self.cofactor(0, col);
        }
        det
    }

    /// Calculates the cofactor of the matrix at the specified position.
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let res: f64 = self.minor(row, col);
        match (col + row) % 2 {
            0 => res,
            _ => -res,
        }
    }

    /// Calculates the minor of the matrix at the specified position.
    ///
    /// The minor is the determinant of the submatrix obtained by removing
    /// the specified row and column.
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        match self.size() {
            2 => self.determinant(),
            3 => self.minor_3x3(row, col),
            4 => self.minor_4x4(row, col),
            _ => panic!("Invalide size of matrix: {}", self.size()),
        }
    }

    /// Helper function to calculate minor for 3x3 matrices.
    fn minor_3x3(&self, row: usize, col: usize) -> f64 {
        let sub_m: Matrix<2> = self.submatrix(row, col);
        sub_m.determinant()
    }

    /// Helper function to calculate minor for 4x4 matrices.
    fn minor_4x4(&self, row: usize, col: usize) -> f64 {
        let sub_m: Matrix<3> = self.submatrix(row, col);
        sub_m.determinant()
    }
}

impl Matrix<4> {
    /// Creates a translation transformation matrix.
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut res: Matrix<4> = Matrix::<4>::identity();
        res[0][3] = x;
        res[1][3] = y;
        res[2][3] = z;
        res
    }

    /// Creates a scaling transformation matrix.
    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut res: Matrix<4> = Matrix::<4>::identity();
        res[0][0] = x;
        res[1][1] = y;
        res[2][2] = z;
        res
    }

    /// Creates a rotation transformation matrix around the X axis.
    pub fn rotation_x(radians: f64) -> Self {
        let mut res: Matrix<4> = Matrix::<4>::identity();
        res[1][1] = radians.cos();
        res[1][2] = -radians.sin();
        res[2][1] = radians.sin();
        res[2][2] = radians.cos();
        res
    }

    /// Creates a rotation transformation matrix around the Y axis.
    pub fn rotation_y(radians: f64) -> Self {
        let mut res: Matrix<4> = Matrix::<4>::identity();
        res[0][0] = radians.cos();
        res[0][3] = radians.sin();
        res[2][0] = -radians.sin();
        res[2][2] = radians.cos();
        res
    }

    /// Creates a rotation transformation matrix around the Z axis.
    pub fn rotation_z(radians: f64) -> Self {
        let mut res: Matrix<4> = Matrix::<4>::identity();
        res[0][0] = radians.cos();
        res[0][1] = -radians.sin();
        res[1][0] = radians.sin();
        res[1][1] = radians.cos();
        res
    }

    /// Creates a shearing transformation matrix.
    ///
    /// Arguments:
    /// * `xy` - Shear factor for X in proportion to Y.
    /// * `xz` - Shear factor for X in proportion to Z.
    /// * `yx` - Shear factor for Y in proportion to X.
    /// * `yz` - Shear factor for Y in proportion to Z.
    /// * `zx` - Shear factor for Z in proportion to X.
    /// * `zy` - Shear factor for Z in proportion to Y.
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut res: Matrix<4> = Matrix::<4>::identity();
        res[0][1] = xy;
        res[0][2] = xz;
        res[1][0] = yx;
        res[1][2] = yz;
        res[2][0] = zx;
        res[2][1] = zy;
        res
    }
}

impl<const N: usize> From<[[f64; N]; N]> for Matrix<N> {
    /// Converts a 2D array to a Matrix.
    fn from(arr: [[f64; N]; N]) -> Self {
        Self { content: arr }
    }
}

impl From<[Tuple; 3]> for Matrix<3> {
    /// Converts an array of 3 Tuples to a 3x3 Matrix.
    fn from(tupl: [Tuple; 3]) -> Self {
        Self {
            content: [tupl[0].to_array(), tupl[1].to_array(), tupl[2].to_array()],
        }
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f64; N];

    /// Provides indexed access to matrix rows.
    fn index(&self, row: usize) -> &Self::Output {
        &self.content[row]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    /// Provides mutable indexed access to matrix rows.
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.content[row]
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    /// Compares two matrices for equality with floating-point precision.
    ///
    /// Uses approximate equality for floating-point values.
    fn eq(&self, other: &Self) -> bool {
        for row in 0..N {
            for col in 0..N {
                if !approx_eq(self[row][col], other[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

impl<const N: usize> Mul<Matrix<N>> for Matrix<N> {
    type Output = Self;

    /// Implements matrix multiplication.
    fn mul(self, other: Matrix<N>) -> Self {
        let mut res = Matrix::<N>::new();

        for row in 0..N {
            for col in 0..N {
                for i in 0..N {
                    res[row][col] += self[row][i] * other[i][col];
                }
            }
        }

        res
    }
}

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    /// Implements matrix multiplication with a Tuple.
    fn mul(self, other: Tuple) -> Self::Output {
        let mut res = Tuple::vector(0.0, 0.0, 0.0);

        res.x = self[0][0] * other.x
            + self[0][1] * other.y
            + self[0][2] * other.z
            + self[0][3] * other.w;
        res.y = self[1][0] * other.x
            + self[1][1] * other.y
            + self[1][2] * other.z
            + self[1][3] * other.w;
        res.z = self[2][0] * other.x
            + self[2][1] * other.y
            + self[2][2] * other.z
            + self[2][3] * other.w;
        res.w = self[3][0] * other.x
            + self[3][1] * other.y
            + self[3][2] * other.z
            + self[3][3] * other.w;

        res
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_new() {
        let m = Matrix::<3>::new();
        assert_eq!(m[0][0], 0.0);
        assert_eq!(m[0][1], 0.0);
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[1][0], 0.0);
        assert_eq!(m[1][1], 0.0);
        assert_eq!(m[1][2], 0.0);
        assert_eq!(m[2][0], 0.0);
        assert_eq!(m[2][1], 0.0);
        assert_eq!(m[2][2], 0.0);

        assert_eq!(m.size(), 3);
    }

    #[test]
    fn test_from_array() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m.get(0, 0), Some(1.0));
        assert_eq!(m.get(1, 3), Some(8.5));
        assert_eq!(m.get(2, 2), Some(11.0));
        assert_eq!(m.get(3, 1), Some(14.5));

        assert_eq!(m.size(), 4);
    }

    #[test]
    fn test_from_tuple() {
        let m = Matrix::from([
            Tuple::point(-3.0, 5.0, 0.0),
            Tuple::point(1.0, -2.0, -7.0),
            Tuple::point(0.0, 1.0, 1.0),
        ]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);

        assert_eq!(m.size(), 3);
    }

    #[test]
    fn test_equality() {
        let m1 = Matrix::from([[0.0, 1.0], [2.0, 3.0]]);
        let m2 = Matrix::from([[0.0, 1.0], [2.0, 3.0]]);
        let m3 = Matrix::from([[0.0, 1.5], [2.0, 3.0]]);

        assert_eq!(m1, m2);
        assert_ne!(m2, m3);
    }

    #[test]
    fn test_index_mut() {
        let mut m = Matrix::<2>::new();

        assert_eq!(m[0][0], 0.0);
        m[0][0] = 1.0;
        assert_eq!(m[0][0], 1.0);
    }

    #[test]
    fn test_get_out_of_bounds() {
        let m = Matrix::<2>::new();

        let res = m.get(2, 0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_multiplication() {
        let m1 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let m3 = m1 * m2;

        assert_eq!(m3[0][0], 20.0);
        assert_eq!(m3[0][1], 22.0);
        assert_eq!(m3[0][2], 50.0);
        assert_eq!(m3[0][3], 48.0);
        assert_eq!(m3[1][0], 44.0);
        assert_eq!(m3[1][1], 54.0);
        assert_eq!(m3[1][2], 114.0);
        assert_eq!(m3[1][3], 108.0);
        assert_eq!(m3[2][0], 40.0);
        assert_eq!(m3[2][1], 58.0);
        assert_eq!(m3[2][2], 110.0);
        assert_eq!(m3[2][3], 102.0);
        assert_eq!(m3[3][0], 16.0);
        assert_eq!(m3[3][1], 26.0);
        assert_eq!(m3[3][2], 46.0);
        assert_eq!(m3[3][3], 42.0);
    }

    #[test]
    fn test_multiplication_to_tuple() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let tup = Tuple::point(1.0, 2.0, 3.0);

        let res = m * tup;

        assert_eq!(res.x, 18.0);
        assert_eq!(res.y, 24.0);
        assert_eq!(res.z, 33.0);
        assert_eq!(res.w, 1.0);
    }

    #[test]
    fn test_identity() {
        let m = Matrix::<4>::identity();

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 0.0);
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[0][3], 0.0);

        assert_eq!(m[1][0], 0.0);
        assert_eq!(m[1][1], 1.0);
        assert_eq!(m[1][2], 0.0);
        assert_eq!(m[1][3], 0.0);

        assert_eq!(m[2][0], 0.0);
        assert_eq!(m[2][1], 0.0);
        assert_eq!(m[2][2], 1.0);
        assert_eq!(m[2][3], 0.0);

        assert_eq!(m[3][0], 0.0);
        assert_eq!(m[3][1], 0.0);
        assert_eq!(m[3][2], 0.0);
        assert_eq!(m[3][3], 1.0);
    }

    #[test]
    fn test_transpose() {
        let m1 = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let m2 = Matrix::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        let m3 = Matrix::<4>::identity();
        let m4 = m3.transpose();

        assert_eq!(m1.transpose(), m2);
        assert_eq!(m3, m4);
    }

    #[test]
    fn test_minor() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let res = m.minor(1, 0);
        assert_eq!(res, 25.0);
    }

    #[test]
    fn test_cofactor() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let minor: f64 = m.minor(0, 0);
        let cofactor: f64 = m.cofactor(0, 0);
        assert_eq!(minor, cofactor);
        assert_eq!(cofactor, -12.0);

        let minor: f64 = m.minor(1, 0);
        let cofactor: f64 = m.cofactor(1, 0);
        assert_ne!(minor, cofactor);
        assert_eq!(cofactor, -25.0)
    }

    #[test]
    fn test_submatrix() {
        let m = Matrix::from([[1., 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let res = m.submatrix::<2>(0, 2);
        let expected = Matrix::from([[-3.0, 2.0], [0.0, 6.0]]);

        assert_eq!(res, expected);

        let m = Matrix::from([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let res = m.submatrix::<3>(2, 1);
        let expected = Matrix::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

        assert_eq!(res, expected);
    }

    #[test]
    fn test_determinant() {
        let m = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(m.determinant(), 17.0);

        let m = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(m.determinant(), -196.0);

        let m = Matrix::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_is_invertible() {
        let m = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(m.determinant(), -2120.0);
        assert!(m.is_invertible());

        let m = Matrix::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(m.determinant(), 0.0);
        assert!(!m.is_invertible());
    }

    #[test]
    fn test_inverse() {
        let a = Matrix::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let inverse_a = Matrix::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        let res = a.inverse();

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, inverse_a);
        assert!(approx_eq(res[0][0], -0.15385));

        let b = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        let c = a * b;
        let inverse_b = b.inverse();
        assert!(inverse_b.is_ok());
        let inverse_b = inverse_b.unwrap();
        let a_restored = c * inverse_b;
        assert_eq!(a_restored, a);
    }

    #[test]
    fn test_translation() {
        let m = Matrix::<4>::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let res = m * p;
        assert_eq!(res, Tuple::point(2.0, 1.0, 7.0));
        let inv = m.inverse();
        assert!(inv.is_ok());
        let inv = inv.unwrap();
        let res = inv * p;
        assert_eq!(res, Tuple::point(-8.0, 7.0, 3.0));
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        let res = m * v;
        assert_eq!(res, v);
    }

    #[test]
    fn test_scaling() {
        let m = Matrix::<4>::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        let res = m * p;
        assert_eq!(res, Tuple::point(-8.0, 18.0, 32.0));
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        let res = m * v;
        assert_eq!(res, Tuple::vector(-8.0, 18.0, 32.0));
        let inv = m.inverse();
        assert!(inv.is_ok());
        let inv = inv.unwrap();
        let res = inv * v;
        assert_eq!(res, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_rotation_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        let half_res = half_quarter * p;
        let full_res = full_quarter * p;
        assert_eq!(
            half_res,
            Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_res, Tuple::point(0.0, 0.0, 1.0));
        let inv = half_quarter.inverse();
        assert!(inv.is_ok());
        let inv = inv.unwrap();
        let res = inv * p;
        assert_eq!(
            res,
            Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        )
    }

    #[test]
    fn test_rotation_y() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);
        let half_res = half_quarter * p;
        let full_res = full_quarter * p;
        assert_eq!(
            half_res,
            Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_res, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotation_z() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);
        let half_res = half_quarter * p;
        let full_res = full_quarter * p;
        assert_eq!(
            half_res,
            Tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_res, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing() {
        let m = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let res = m * p;
        assert_eq!(res, Tuple::point(5.0, 3.0, 4.0));
        let m = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let res = m * p;
        assert_eq!(res, Tuple::point(6.0, 3.0, 4.0));
        let m = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let res = m * p;
        assert_eq!(res, Tuple::point(2.0, 5.0, 4.0));
        let m = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let res = m * p;
        assert_eq!(res, Tuple::point(2.0, 7.0, 4.0));
        let m = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let res = m * p;
        assert_eq!(res, Tuple::point(2.0, 3.0, 6.0));
        let m = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let res = m * p;
        assert_eq!(res, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_chaining_transformations() {
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let p = Tuple::point(1.0, 0.0, 1.0);
        let res = a * p;
        assert_eq!(res, Tuple::point(1.0, -1.0, 0.0));
        let res = b * res;
        assert_eq!(res, Tuple::point(5.0, -5.0, 0.0));
        let res = c * res;
        assert_eq!(res, Tuple::point(15.0, 0.0, 7.0));
        let transform = c * b * a;
        let res = transform * p;
        assert_eq!(res, Tuple::point(15.0, 0.0, 7.0));
    }
}
