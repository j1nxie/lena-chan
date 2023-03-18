use crate::tuple::Tuple;
use std::{
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
