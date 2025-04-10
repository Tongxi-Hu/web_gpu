use std::ops::{Index, IndexMut, Mul};

use super::common::FuzzyEq;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const D: usize> {
    data: [[f32; D]; D],
}

impl<const D: usize> Index<usize> for Matrix<D> {
    type Output = [f32; D];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const D: usize> IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const D: usize> Matrix<D> {
    pub fn new() -> Self {
        Self {
            data: [[0.0; D]; D],
        }
    }

    pub fn diagonal(value: f32) -> Self {
        let mut matrix = Self::new();
        for i in 0..D {
            matrix[i][i] = value;
        }
        matrix
    }

    pub fn identity() -> Self {
        Self::diagonal(1.0)
    }

    pub fn transpose(&self) -> Self {
        let mut matrix = Self::new();
        for row in 0..D {
            for col in 0..D {
                matrix[col][row] = self[row][col]
            }
        }
        matrix
    }
}

impl<const D: usize> Default for Matrix<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const D: usize> FuzzyEq for Matrix<D> {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        for row in 0..D {
            for col in 0..D {
                if self[row][col].fuzzy_eq(&other[row][col]) == false {
                    return false;
                }
            }
        }
        true
    }
}

impl<const D: usize> PartialEq for Matrix<D> {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}

impl<const D: usize> Mul for Matrix<D> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Self::new();
        for row in 0..D {
            for col in 0..D {
                for inter in 0..D {
                    res[row][col] = res[row][col] + self[row][inter] * rhs[inter][col];
                }
            }
        }
        self
    }
}
