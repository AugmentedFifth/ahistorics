//! Thin wrapper around the `Matrix2d` type provided and accepted by piston's
//! graphics API. Provides convenience methods and operator overloads. Also
//! implements `Deref` and `DerefMut` targetting `Matrix2d`.

use graphics::math;
use graphics::math::{
    get_scale,
    identity,
    Matrix2d,
    multiply,
    rotate_radians,
    translate,
    Vec2d,
};

use std::ops::{
    Add,
    AddAssign,
    Deref,
    DerefMut,
    Index,
    IndexMut,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

use vecmath::{
    mat2x3_add,
    mat2x3_det,
    mat2x3_inv,
    mat2x3_sub,
    row_mat2x3_transform_pos2,
    row_mat2x3_transform_vec2,
};


#[derive(Copy)]
pub struct Matrix {
    pub repr: Matrix2d,
}


impl Matrix {
    /// Creates a new matrix, wrapping a single `Matrix2d`.
    pub fn new(m: Matrix2d) -> Self {
        Matrix { repr: m }
    }

    /// Same as `::new()`, creates a new matrix that is the identity matrix.
    pub fn identity() -> Self {
        Matrix { repr: identity() }
    }

    /// Extracts the scaling components of this matrix as a 2D vector.
    pub fn get_scale(&self) -> Vec2d {
        get_scale(self.repr)
    }

    /// Get the determinant of this matrix.
    pub fn det(&self) -> f64 {
        mat2x3_det(self.repr)
    }

    /// Get the inverse of this matrix.
    pub fn inv(&self) -> Self {
        Matrix { repr: mat2x3_inv(self.repr) }
    }

    /// Get a new matrix that is this matrix scaled by the given scalar `k`.
    pub fn scalar_mul(&self, k: f64) -> Self {
        Matrix {
            repr: [
                [
                    k * self.repr[0][0],
                    k * self.repr[0][1],
                    k * self.repr[0][2],
                ],
                [
                    k * self.repr[1][0],
                    k * self.repr[1][1],
                    k * self.repr[1][2],
                ],
            ],
        }
    }

    /// Modifies this matrix in place to scale it by the given scalar `k`.
    pub fn scalar_mul_in_place(&mut self, k: f64) {
        for a in 0..2 {
            for b in 0..3 {
                self.repr[a][b] *= k;
            }
        }
    }

    /// Get a 2D vector that is the result of the given vector being
    /// transformed by this matrix.
    pub fn vec_mul(&self, v: Vec2d) -> Vec2d {
        row_mat2x3_transform_vec2(self.repr, v)
    }

    /// Get a 2D vector (representing a point) that is the result of the given
    /// vector (representing the same kind of point) being transformed by this
    /// matrix.
    pub fn pos_mul(&self, p: Vec2d) -> Vec2d {
        row_mat2x3_transform_pos2(self.repr, p)
    }
}

impl Deref for Matrix {
    type Target = Matrix2d;

    fn deref(&self) -> &Self::Target {
        &self.repr
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.repr
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        *self
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (a, b) = index;
        &self.repr[a][b]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (a, b) = index;
        &mut self.repr[a][b]
    }
}

impl Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Matrix {
            repr: [[-self.repr[0][0], -self.repr[0][1], -self.repr[0][2]],
                   [-self.repr[1][0], -self.repr[1][1], -self.repr[1][2]]]
        }
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix { repr: mat2x3_add(self.repr, rhs.repr) }
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        self.repr = mat2x3_add(self.repr, rhs.repr);
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix { repr: mat2x3_sub(self.repr, rhs.repr) }
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        for a in 0..2 {
            for b in 0..3 {
                self.repr[a][b] -= rhs.repr[a][b];
            }
        }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix { repr: multiply(rhs.repr, self.repr) }
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        self.repr = multiply(self.repr, rhs.repr);
    }
}

/// Creates a new `Matrix`, wrapping a single `Matrix2d`.
pub fn m(m2d: Matrix2d) -> Matrix {
    Matrix::new(m2d)
}

/// Creates a new `Matrix` representing anticlockwise rotation by the specified
/// number of radians.
pub fn rot(theta: f64) -> Matrix {
    Matrix::new(rotate_radians(theta))
}

/// Creates a new `Matrix` representing scaling by the specified factors along
/// the x and y axes, respectively.
pub fn scale(x: f64, y: f64) -> Matrix {
    Matrix::new(math::scale(x, y))
}

/// Creates a new `Matrix` representing scaling by the specified factor along
/// both axes uniformly.
pub fn scale_uni(k: f64) -> Matrix {
    Matrix::new(math::scale(k, k))
}

/// Creates a new `Matrix` representing translation by the specified 2D vector.
pub fn trans(v: Vec2d) -> Matrix {
    Matrix::new(translate(v))
}
