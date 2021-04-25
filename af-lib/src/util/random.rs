// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Randomness utilities and random number generation.

use std::cell::RefCell;

use parking_lot::Mutex;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::distributions::{self, Distribution};
use rand::seq::SliceRandom;
use rand::{Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256StarStar;

use crate::prelude::*;

/// Returns `true` with a given probability.
///
/// The `probability` is the chance from `0.0` (never) to `1.0` (always) that
/// this function returns `true`.
pub fn chance(probability: f64) -> bool {
  THREAD_RNG.with(|rng| rng.borrow_mut().gen_chance(probability))
}

/// Fills a slice with random bytes.
pub fn fill_bytes(bytes: &mut [u8]) {
  THREAD_RNG.with(|rng| rng.borrow_mut().fill_bytes(bytes))
}

/// Generates a random value.
pub fn random<T: Random>() -> T {
  THREAD_RNG.with(|rng| T::random_with(&mut rng.borrow_mut()))
}

/// Generates a random number within a range.
pub fn range<T: SampleUniform>(range: impl SampleRange<T>) -> T {
  THREAD_RNG.with(|rng| rng.borrow_mut().gen_range(range))
}

/// Returns `true` with a probability expressed by the ratio between two
/// numbers.
pub fn ratio<T: Number + SampleUniform>(numerator: T, denominator: T) -> bool {
  THREAD_RNG.with(|rng| rng.borrow_mut().gen_ratio(numerator, denominator))
}

/// Randomly shuffles a slice in place.
pub fn shuffle<T>(slice: &mut [T]) {
  THREAD_RNG.with(|rng| rng.borrow_mut().shuffle(slice))
}

/// A trait for types that can be initialized randomly.
pub trait Random: Sized {
  /// Returns a random value using the given `Rng`.
  fn random_with(rng: &mut Rng) -> Self;

  /// Returns a random value.
  fn random() -> Self {
    random()
  }
}

// Implement `Random` for all types that can be used with `rng.gen()`.

impl<T> Random for T
where
  distributions::Standard: Distribution<T>,
{
  fn random_with(rng: &mut Rng) -> Self {
    rng.inner.gen()
  }
}

/// A random number generator.
#[derive(Clone)]
pub struct Rng {
  inner: Xoshiro256StarStar,
}

/// The global RNG to use to create thread-local RNGs.
static GLOBAL_RNG: Lazy<Mutex<Rng>> =
  Lazy::new(|| Mutex::new(Rng { inner: Xoshiro256StarStar::from_entropy() }));

thread_local! {
  /// The thread-local RNG.
  static THREAD_RNG: RefCell<Rng> = {
    let mut global_rng = GLOBAL_RNG.lock();
    let thread_rng = global_rng.clone();

    global_rng.inner.long_jump();

    RefCell::new(thread_rng)
  };
}

impl Rng {
  /// Creates a new `Rng` with a random seed.
  pub fn new() -> Rng {
    THREAD_RNG.with(|rng| {
      let mut thread_rng = rng.borrow_mut();
      let local_rng = thread_rng.clone();

      thread_rng.inner.jump();

      local_rng
    })
  }

  /// Fills a slice with random bytes.
  pub fn fill_bytes(&mut self, bytes: &mut [u8]) {
    self.inner.fill(bytes);
  }

  /// Generates a random value.
  pub fn gen<T: Random>(&mut self) -> T {
    T::random_with(self)
  }

  /// Returns `true` with a given probability.
  ///
  /// The probability is the chance from `0.0` (never) to `1.0` (always) that
  /// this function returns `true`.
  pub fn gen_chance(&mut self, probability: f64) -> bool {
    probability > self.gen()
  }

  /// Generates a random number within a given range.
  pub fn gen_range<T: SampleUniform>(&mut self, range: impl SampleRange<T>) -> T {
    self.inner.gen_range(range)
  }

  /// Returns `true` with a probability expressed by the ratio between two given
  /// numbers.
  pub fn gen_ratio<T: Number + SampleUniform>(&mut self, numerator: T, denominator: T) -> bool {
    debug_assert!(denominator > T::zero(), "The denominator of a ratio must be greater than zero.");

    numerator > self.gen_range(T::zero()..denominator)
  }

  /// Randomly shuffles a slice in place.
  pub fn shuffle<T>(&mut self, slice: &mut [T]) {
    slice.shuffle(&mut self.inner);
  }
}

impl Default for Rng {
  fn default() -> Self {
    Self::new()
  }
}
