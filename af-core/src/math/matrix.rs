// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Two-dimensional matrices.

use super::*;

/// A 4x4 matrix of `T`.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Matrix4<T> {
  pub columns: [Vector4<T>; 4],
}

impl<T: Number + Copy> Matrix4<T> {
  /// Creates a new matrix from a set of column vectors.
  pub fn new(
    column0: Vector4<T>,
    column1: Vector4<T>,
    column2: Vector4<T>,
    column3: Vector4<T>,
  ) -> Self {
    Self { columns: [column0, column1, column2, column3] }
  }

  /// Creates a new matrix from a three-dimensional scaling factor.
  pub fn scale(scale: Vector3<T>) -> Self {
    Matrix4::new(
      Vector4::new(scale.x, zero(), zero(), zero()),
      Vector4::new(zero(), scale.y, zero(), zero()),
      Vector4::new(zero(), zero(), scale.z, zero()),
      Vector4::new(zero(), zero(), zero(), one()),
    )
  }

  /// Returns a [`Vector4`] containing the components in a row.
  pub fn row(&self, index: usize) -> Vector4<T> {
    Vector4::new(
      self.columns[0][index],
      self.columns[1][index],
      self.columns[2][index],
      self.columns[3][index],
    )
  }

  /// Returns an array of [`Vector4`] containing the components of each row.
  pub fn rows(&self) -> [Vector4<T>; 4] {
    [self.row(0), self.row(1), self.row(2), self.row(3)]
  }
}

impl Matrix4<f32> {
  /// Creates a new orthographic projection matrix of the given two-dimensional
  /// size.
  pub fn orthographic_projection(size: Vector2<f32>) -> Self {
    let c0r0 = 2.0 / size.x;
    let c1r1 = 2.0 / size.y;

    Matrix4::new(
      Vector4::new(c0r0, 0.0, 0.0, 0.0),
      Vector4::new(0.0, c1r1, 0.0, 0.0),
      Vector4::new(0.0, 0.0, 0.0, 0.0),
      Vector4::new(0.0, 0.0, 0.0, 1.0),
    )
  }
}

impl<T: Number + Copy> Default for Matrix4<T> {
  fn default() -> Self {
    one()
  }
}

impl<T: Number + Copy> Mul<Self> for Matrix4<T> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    let rows = rhs.rows();

    Matrix4::new(
      Vector4::new(
        self.columns[0] * rows[0],
        self.columns[1] * rows[0],
        self.columns[2] * rows[0],
        self.columns[3] * rows[0],
      ),
      Vector4::new(
        self.columns[0] * rows[1],
        self.columns[1] * rows[1],
        self.columns[2] * rows[1],
        self.columns[3] * rows[1],
      ),
      Vector4::new(
        self.columns[0] * rows[2],
        self.columns[1] * rows[2],
        self.columns[2] * rows[2],
        self.columns[3] * rows[2],
      ),
      Vector4::new(
        self.columns[0] * rows[3],
        self.columns[1] * rows[3],
        self.columns[2] * rows[3],
        self.columns[3] * rows[3],
      ),
    )
  }
}

impl<T: Number + Copy> MulAssign<Self> for Matrix4<T> {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl<T: Number + Copy> One for Matrix4<T> {
  fn one() -> Self {
    Self {
      columns: [
        Vector4::new(one(), zero(), zero(), zero()),
        Vector4::new(zero(), one(), zero(), zero()),
        Vector4::new(zero(), zero(), one(), zero()),
        Vector4::new(zero(), zero(), zero(), one()),
      ],
    }
  }
}
