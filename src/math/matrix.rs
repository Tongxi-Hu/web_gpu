use std::ops::{Index, IndexMut, Mul};

use super::common::{Determinant, FuzzyEq};

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
        res
    }
}

impl Determinant for Matrix<2> {
    fn det(&self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Matrix<3> {
    pub fn translate(tx: f32, ty: f32) -> Self {
        let mut trans = Self::identity();
        trans.data[0][2] = tx;
        trans.data[1][2] = ty;
        trans
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        let mut scale = Self::identity();
        scale[0][0] = sx;
        scale[1][1] = sy;
        scale
    }

    pub fn rotate(deg: f32) -> Self {
        let mut rotation = Self::identity();
        let (cos, sin) = (deg.cos(), deg.sin());
        rotation[0][0] = cos;
        rotation[1][1] = cos;
        rotation[0][1] = -sin;
        rotation[1][0] = sin;
        rotation
    }
}

impl Determinant for Matrix<3> {
    fn det(&self) -> f32 {
        self[0][0] * self[1][1] * self[2][2]
            + self[0][1] * self[1][2] * self[2][1]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][2] * self[1][1] * self[2][1]
            - self[0][1] * self[1][0] * self[2][2]
            - self[0][0] * self[1][2] * self[2][1]
    }
}

impl Matrix<4> {
    fn sub(&self, row: usize, column: usize) -> Matrix<3> {
        let mut matrix: Matrix<3> = Matrix::new();
        let mut source_row: usize = 0;
        let mut source_column: usize = 0;
        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < 3 {
            if source_row == row {
                // Skip row to be removed
                source_row += 1;
            }
            while target_column < 3 {
                if source_column == column {
                    // Skip column to be removed
                    source_column += 1;
                }
                matrix[target_row][target_column] = self[source_row][source_column];

                source_column += 1;
                target_column += 1;
            }
            source_row += 1;
            source_column = 0;
            target_row += 1;
            target_column = 0;
        }

        matrix
    }

    fn co_factor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.sub(row, col).det()
        } else {
            -self.sub(row, col).det()
        }
    }

    pub fn is_invertible(&self) -> bool {
        !self.det().fuzzy_eq(&0.0)
    }

    pub fn inverse(&self) -> Result<Matrix<4>, String> {
        if self.is_invertible() {
            let det = self.det();
            let mut inverse: Matrix<4> = Matrix::new();
            for row in 0..4 {
                for col in 0..4 {
                    let cofactor = self.co_factor(row, col);
                    inverse[col][row] = cofactor / det;
                }
            }
            Ok(inverse)
        } else {
            Err(String::from("not invertible"))
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Matrix<4> {
        Matrix::<4> {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix<4> {
        Matrix::<4> {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_x(r: f32) -> Matrix<4> {
        Matrix::<4> {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, r.cos(), -r.sin(), 0.0],
                [0.0, r.sin(), r.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_y(r: f32) -> Matrix<4> {
        Matrix::<4> {
            data: [
                [r.cos(), 0.0, r.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-r.sin(), 0.0, r.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_z(r: f32) -> Matrix<4> {
        Matrix::<4> {
            data: [
                [r.cos(), -r.sin(), 0.0, 0.0],
                [r.sin(), r.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Determinant for Matrix<4> {
    fn det(&self) -> f32 {
        let mut det: f32 = 0.0;
        for col in 0..4 {
            det = det + self.co_factor(0, col) * self[0][col]
        }
        det
    }
}
