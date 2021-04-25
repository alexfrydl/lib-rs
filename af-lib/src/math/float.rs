// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// An extension trait for `f32` and `f64`.
pub trait FloatExt {
  /// Rounds the number up to a specified number of decimal places.
  ///
  /// Calling `ceil_to_places(0)` is equivalent to calling `ceil()`.
  fn ceil_to_places(self, places: usize) -> Self;

  /// Rounds the number down to a specified number of decimal places.
  ///
  /// Calling `floor_to_places(0)` is equivalent to calling `floor()`.
  fn floor_to_places(self, places: usize) -> Self;

  /// Rounds the number to a specified number of decimal places.
  ///
  /// Calling `round_to_places(0)` is equivalent to calling `round()`.
  fn round_to_places(self, places: usize) -> Self;
}

/// Returns a multiplier representing a number of decimal places.
fn places_f32(places: usize) -> f32 {
  debug_assert!(places < i32::MAX as usize, "`places` must be less than {}", i32::MAX);

  match places {
    0 => 1.0,
    1 => 10.0,
    2 => 100.0,
    3 => 1000.0,
    n => 10.0f32.powi(n as i32),
  }
}

/// Returns a multiplier representing a number of decimal places.
fn places_f64(places: usize) -> f64 {
  debug_assert!(places < i32::MAX as usize, "`places` must be less than {}", i32::MAX);

  match places {
    0 => 1.0,
    1 => 10.0,
    2 => 100.0,
    3 => 1000.0,
    n => 10.0f64.powi(n as i32),
  }
}

impl FloatExt for f32 {
  fn ceil_to_places(self, places: usize) -> Self {
    let mult = places_f32(places);

    (self * mult).ceil() / mult
  }

  fn floor_to_places(self, places: usize) -> Self {
    let mult = places_f32(places);

    (self * mult).floor() / mult
  }

  fn round_to_places(self, places: usize) -> Self {
    let mult = places_f32(places);

    (self * mult).round() / mult
  }
}

impl FloatExt for f64 {
  fn ceil_to_places(self, places: usize) -> Self {
    let mult = places_f64(places);

    (self * mult).ceil() / mult
  }

  fn floor_to_places(self, places: usize) -> Self {
    let mult = places_f64(places);

    (self * mult).floor() / mult
  }

  fn round_to_places(self, places: usize) -> Self {
    let mult = places_f64(places);

    (self * mult).round() / mult
  }
}
