// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::channel;
use crate::prelude::*;
use crate::task;
use crate::test::Path;

/// A stream of test results.
pub struct OutputStream {
  pub(super) remaining: usize,
  pub(super) rx: channel::Receiver<Output>,
  pub(super) _task: task::Handle<(), Infallible>,
}

/// A single test result.
#[derive(Debug)]
pub struct Output {
  /// The path of the test, including scope names.
  pub path: Path,
  /// The result of the test.
  pub result: fail::Result,
}

impl OutputStream {
  /// Returns `true` if no tests remain.
  pub fn is_empty(&self) -> bool {
    self.remaining == 0
  }

  /// Returns the number of tests remaining.
  pub fn len(&self) -> usize {
    self.remaining
  }

  /// Waits for the next test to complete and returns its result.
  ///
  /// If all tests have completed, this function returns `None`.
  pub async fn next(&mut self) -> Option<Output> {
    let result = self.rx.recv().await.ok();

    if result.is_some() {
      self.remaining -= 1;
    } else if self.remaining > 0 {
      panic!("OutputStream closed with {} remaining tasks.", self.remaining);
    }

    result
  }
}
