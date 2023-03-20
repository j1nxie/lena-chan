use crate::tuple::Tuple;
use float_eq::float_eq;
use std::{
    f64::EPSILON,
    ops::{Add, Index, IndexMut, Mul, Sub},
};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(width: usize, height: usize, data: Vec<f64>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn identity(&self) -> Self {
        let mut data = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                if x == y {
                    data.push(1.0);
                } else {
                    data.push(0.0);
                }
            }
        }

        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn transpose(&self) -> Self {
        let mut data = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                data.push(self[(x, y)]);
            }
        }

        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn determinant(&self) -> f64 {
        if self.width == 2 && self.height == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            todo!()
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut data = self.data.clone();

        for (i, x) in (0..self.height).enumerate() {
            data.remove(col + self.width * x - i);
        }

        for (i, y) in (0..self.width - 1).enumerate() {
            data.remove(row * (self.width - 1) + y - i);
        }

        Self {
            width: self.width - 1,
            height: self.height - 1,
            data: data.to_vec(),
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let result = (self.width == other.width) && (self.height == other.height);
        if !result {
            return result;
        }
        self.data
            .iter()
            .zip(other.data.iter())
            .fold(true, |acc, (x, y)| acc && float_eq!(x, y, abs <= EPSILON))
    }
}

impl Eq for Matrix {}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Self) -> Self {
        if self.width != other.width || self.height != other.height {
            panic!("cannot add two matrices of different dimensions");
        }

        let result: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(x, y)| x + y)
            .collect();

        Self {
            width: self.width,
            height: self.height,
            data: result,
        }
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Self) -> Self {
        if self.width != other.width || self.height != other.height {
            panic!("cannot subtract two matrices of different dimensions");
        }

        let result: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(x, y)| x - y)
            .collect();

        Self {
            width: self.width,
            height: self.height,
            data: result,
        }
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, other: f64) -> Self {
        Self {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|x| x * other).collect(),
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self {
        if self.height != other.width {
            panic!("number of columns in the first matrix should be equal to number of rows in the second matrix!");
        }

        let mut result = vec![];

        for i in 0..self.width {
            for j in 0..other.height {
                let mut sum = 0.0;
                for k in 0..self.height {
                    sum += self[(i, k)] * other[(k, j)]
                }
                result.push(sum);
            }
        }

        Self {
            width: self.width,
            height: other.height,
            data: result,
        }
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        if self.height != 4 {
            panic!("cannot multiply this matrix with a tuple!");
        }

        let tuple_matrix = Matrix::new(4, 1, vec![other.x, other.y, other.z, other.w]);

        let result = self * tuple_matrix;

        Tuple::new(
            result[(0, 0)],
            result[(1, 0)],
            result[(2, 0)],
            result[(3, 0)],
        )
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        match self.data.get(col + row * self.height) {
            Some(t) => t,
            None => panic!(
                "out of bounds! tried to get index of ({}, {}) for matrix size ({} {})",
                row, col, self.width, self.height
            ),
        }
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        match self.data.get_mut(col + row * self.height) {
            Some(t) => t,
            None => panic!(
                "out of bounds! tried to get index of ({}, {}) for matrix size ({} {})",
                row, col, self.width, self.height
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_matrix() {
        let matrix = Matrix::size(2, 2);

        assert_eq!(matrix.data, vec![0.0; 4])
    }

    #[test]
    fn test_index_matrix() {
        let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 1)], 2.0);
        assert_eq!(matrix[(1, 0)], 3.0);
        assert_eq!(matrix[(1, 1)], 4.0);
    }

    #[test]
    fn test_add_matrix_ok() {
        let matrix = Matrix::new(2, 2, vec![1.0; 4]);
        let other = Matrix::new(2, 2, vec![1.0; 4]);
        let result = Matrix::new(2, 2, vec![2.0; 4]);

        assert_eq!(matrix + other, result);
    }

    #[test]
    #[should_panic(expected = "cannot add two matrices of different dimensions")]
    fn test_add_matrix_fail() {
        let matrix = Matrix::size(2, 2);
        let other = Matrix::size(3, 2);

        let _ = matrix + other;
    }

    #[test]
    fn test_sub_matrix_ok() {
        let matrix = Matrix::new(2, 2, vec![1.0; 4]);
        let other = Matrix::new(2, 2, vec![1.0; 4]);
        let result = Matrix::new(2, 2, vec![0.0; 4]);

        assert_eq!(matrix - other, result);
    }

    #[test]
    #[should_panic(expected = "cannot subtract two matrices of different dimensions")]
    fn test_sub_matrix_fail() {
        let matrix = Matrix::size(2, 2);
        let other = Matrix::size(3, 2);

        let _ = matrix - other;
    }

    #[test]
    fn test_mul_matrix_scalar() {
        let matrix = Matrix::new(2, 2, vec![1.0, 1.0, 1.0, 1.0]);
        let scalar = 4.0;
        let result = Matrix::new(2, 2, vec![4.0, 4.0, 4.0, 4.0]);

        assert_eq!(matrix * scalar, result);
    }

    #[test]
    fn test_mul_matrices_ok() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let other = Matrix::new(
            4,
            4,
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
        );

        let result = Matrix::new(
            4,
            4,
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
        );

        assert_eq!((matrix * other).data, result.data);
    }

    #[test]
    #[should_panic(
        expected = "number of columns in the first matrix should be equal to number of rows in the second matrix!"
    )]
    fn test_mul_matrices_fail() {
        let matrix = Matrix::size(2, 2);
        let other = Matrix::size(3, 2);

        let _ = matrix * other;
    }

    #[test]
    fn test_mul_matrix_tuple_ok() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(matrix * tuple, result);
    }

    #[test]
    fn test_cmp_matrix() {
        let m1 = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let m2 = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let m3 = Matrix::new(
            4,
            4,
            vec![
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ],
        );

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn test_mul_identity_matrix() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 17.0, 4.0, 8.0, 16.0, 32.0,
            ],
        );
        let identity = matrix.identity();

        assert_eq!(matrix.clone() * identity, matrix);
    }

    #[test]
    fn test_mul_identity_matrix_tuple() {
        let matrix = Matrix::size(4, 4).identity();
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(matrix * tuple, tuple);
    }

    #[test]
    fn test_transpose_matrix() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ],
        );
        let transposed = Matrix::new(
            4,
            4,
            vec![
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
            ],
        );

        assert_eq!(matrix.transpose(), transposed);
    }

    #[test]
    fn test_transpose_identity_matrix() {
        let matrix = Matrix::size(2, 2).identity();

        assert_eq!(matrix.transpose(), matrix);
    }

    #[test]
    fn test_det_matrix_2x2() {
        let matrix = Matrix::new(2, 2, vec![1.0, 5.0, -3.0, 2.0]);
        let determinant = 17.0;

        assert_eq!(matrix.determinant(), determinant);
    }

    #[test]
    fn test_submatrix_3x3() {
        let matrix = Matrix::new(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, -7.0, 0.0, 6.0, -3.0]);
        let submatrix = Matrix::new(2, 2, vec![-3.0, 2.0, 0.0, 6.0]);

        assert_eq!(matrix.submatrix(0, 2), submatrix);
    }

    #[test]
    fn test_submatrix_4x4() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
        );
        let submatrix = Matrix::new(3, 3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);

        assert_eq!(matrix.submatrix(2, 1), submatrix);
    }

    #[test]
    fn test_minor_3x3() {
        let matrix = Matrix::new(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        let submatrix = matrix.submatrix(1, 0);

        assert_eq!(matrix.minor(1, 0), submatrix.determinant())
    }
}
