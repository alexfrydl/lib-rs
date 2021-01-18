// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A default test runner with a progress bar and formatted output.

pub use af_macros::test_main as main;

use crate::test::prelude::*;
use crate::util::defer;
use console::style;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::io;

/// A test runner error.
#[derive(Debug, Error)]
pub enum Error {
  /// One or more tests failed.
  #[error("{}.", fmt::count(*.0, "test failure", "test failures"))]
  Failures(usize),
  /// An IO error occurred writing to `stdout`.
  #[error("IO error. {0}")]
  Io(#[from] io::Error),
}

/// The output of the test runner.
pub struct Output {
  pub elapsed: Duration,
  pub failures: usize,
  pub tests: usize,
}

/// A test runner result.
pub type Result<T = Output, E = Error> = std::result::Result<T, E>;

/// Runs a test context in the default test runner.
pub async fn run(build: impl FnOnce(&mut test::Context)) -> Result {
  let style_initial = ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40} {pos:>7}/{len:7} {msg}")
    .progress_chars("##-");

  let style_ok = style_initial
    .clone()
    .template("[{elapsed_precise}] {bar:40.green.bright/.green} {pos:>7}/{len:7} {msg}");

  let style_err = style_ok
    .clone()
    .template("[{elapsed_precise}] {bar:40.red.bright/.red} {pos:>7}/{len:7} {msg}");

  let mut term = console::Term::buffered_stdout();
  let pb = ProgressBar::with_draw_target(0, ProgressDrawTarget::to_term(term.clone(), None));

  pb.set_message("Starting…");
  pb.set_style(style_initial);

  let mut ctx = test::Context::new();

  build(&mut ctx);

  let panic_hook = panic::take_hook();
  let _guard = defer(|| panic::set_hook(panic_hook));

  panic::set_hook(Box::new(|_| ()));

  let started_at = Time::now();
  let mut output = ctx.start();

  let tests = output.len();
  let mut failures = 0;

  pb.set_length(tests as u64);
  pb.set_message("Running…");
  pb.set_style(style_ok);

  while let Some(test::Output { path, result }) = output.next().await {
    if let Err(err) = result {
      failures += 1;

      term.clear_last_lines(1)?;

      pb.set_style(style_err.clone());

      writeln!(
        term,
        "{} {} — {:#}\n",
        path,
        style("failed").bright().red(),
        fmt::indent("", "  ", err)
      )?;
    }

    pb.set_position((tests - output.len()) as u64);
  }

  pb.finish_and_clear();

  let elapsed = started_at.elapsed();

  let (count, status) = match failures {
    0 => (fmt::count(tests, "test", "tests"), style("passed").bright().green()),
    n => (fmt::count(n, "test", "tests"), style("failed").bright().red()),
  };

  if failures > 0 {
    writeln!(term)?;
  }

  writeln!(term, "{} {} in {}.", count, status, style(elapsed).bright().white())?;

  term.flush()?;

  Ok(Output { elapsed, failures, tests })
}
