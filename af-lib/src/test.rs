// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Asynchronous test suites that run tests on separate tasks.

/// A version of the [prelude][crate::prelude] that includes the `test` module.
pub mod prelude {
  pub use crate::prelude::*;
  pub use crate::test;
}

use std::panic::AssertUnwindSafe;

pub use af_macros::test_main as main;
use console::style;

use crate::concurrency::{channel, future, scope, task};
use crate::prelude::*;
use crate::util::SharedStr;

/// Runs a test suite and waits for all tests to complete.
pub async fn run_suite<F>(suite: impl FnOnce(Context) -> F) -> SuiteOutput
where
  F: Future<Output = ()>,
{
  let (outputs_tx, outputs_rx) = channel().split();
  let root = Root { outputs_tx };

  suite(Context { name: default(), root }).await;

  let mut output: SuiteOutput = default();

  while let Some(test) = outputs_rx.recv().await {
    match test.result {
      Ok(_) => output.passed_count += 1,

      Err(err) => {
        match output.failed_count {
          0 => write!(output.errors, "Test {:?} {}", test.name, err).unwrap(),
          _ => write!(output.errors, "\n\nTest {:?} {}", test.name, err).unwrap(),
        }

        output.failed_count += 1;
      }
    }
  }

  output
}

/// Joins two halves of a test or context name.
fn join_names(prefix: &str, suffix: impl AsRef<str>) -> String {
  let suffix = suffix.as_ref();

  if prefix.is_empty() {
    return suffix.into();
  } else if suffix.is_empty() {
    return prefix.into();
  }

  let has_punctuation = matches!(prefix.chars().last(), Some(c) if c.is_ascii_punctuation())
    || matches!(suffix.chars().next(), Some(c) if c.is_ascii_punctuation());

  if has_punctuation {
    format!("{}{}", prefix, suffix)
  } else {
    format!("{} {}", prefix, suffix)
  }
}

/// A test context.
pub struct Context {
  name: String,
  root: Root,
}

impl Context {
  /// Creates a new child context for grouping related tests.
  pub fn context(&self, name: impl AsRef<str>) -> Self {
    Self { name: join_names(&self.name, name), root: self.root.clone() }
  }

  /// Runs a test by starting it on a separate task.
  pub fn test<O>(&self, name: impl AsRef<str>, test: impl Future<Output = O> + Send + 'static)
  where
    O: scope::IntoOutput,
  {
    let name = SharedStr::from(join_names(&self.name, name));
    let outputs_tx = self.root.outputs_tx.clone();

    let report_drop = defer({
      let name = name.clone();
      let outputs_tx = outputs_tx.clone();

      move || {
        outputs_tx.send(TestOutput {
          name,
          result: Err(scope::Error::Error(
            "The task was canceled before the test completed. You may be missing a `join().await`."
              .into(),
          )),
        });
      }
    });

    task::start_as(name.clone(), async move {
      let result = future::capture_panic(AssertUnwindSafe(test))
        .await
        .map_err(scope::Error::from)
        .and_then(|output| output.into_scope_output().map_err(scope::Error::from));

      outputs_tx.send(TestOutput { name, result });

      report_drop.cancel();
    });
  }
}

/// The root context of the test suite.
#[derive(Clone)]
struct Root {
  outputs_tx: channel::Sender<TestOutput>,
}

/// Output of a test suite.
#[derive(Default)]
pub struct SuiteOutput {
  /// A string containing preformatted error messages for each failed test.
  ///
  /// If no tests failed, this value is empty.
  pub errors: String,
  /// The number of failed tests.
  pub failed_count: usize,
  /// The number of passed tests.
  pub passed_count: usize,
}

impl Display for SuiteOutput {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let failed = fmt::count(self.failed_count, "test", "tests");
    let passed = fmt::count(self.passed_count, "test", "tests");

    let summary = match self.failed_count {
      0 => format!("{} passed", passed),
      _ => format!("{} passed, {} failed", passed, failed),
    };

    if !f.alternate() {
      write!(f, "{}", summary)?;

      if self.failed_count > 0 {
        write!(f, "\n\n{}", self.errors)?;
      }
    } else if self.failed_count > 0 {
      write!(f, "{}\n\n{}", self.errors, style(summary).red().bright())?;
    } else {
      write!(f, "{}\n\n{}", self.errors, style(summary).green().bright())?;
    }

    Ok(())
  }
}

/// The output of a single test.
struct TestOutput {
  name: SharedStr,
  result: Result<(), scope::Error>,
}
