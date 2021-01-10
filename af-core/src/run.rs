use crate::prelude::*;
use crate::task;
use crate::thread;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use std::process::exit;

/// Starts a task from the given future, waits for it to complete, then exits
/// the process.
///
/// If the task fails, this function logs the error and exits the process with
/// a non-zero exit code.
pub fn run<E>(future: impl Future<Output = Result<(), E>> + Send + 'static) -> !
where
  E: Display + Send + 'static,
{
  let task = task::start(future);

  match thread::block_on(task) {
    Err(task::Failure::Panic(err)) => {
      if let Some(value) = err.display_value() {
        error!("The main task panicked with `{}`.", value);
      } else {
        error!("The main task panicked.")
      }

      thread::sleep(Duration::hz(60));
      exit(-1)
    }

    Err(task::Failure::Err(err)) => {
      error!("The main task failed. {}", err);

      thread::sleep(Duration::hz(60));
      exit(1)
    }

    _ => exit(0),
  }
}

/// Starts a task from the given function, waits for it to complete, then
/// exits the process.
///
/// If the task fails, this function logs the error and exits the process with
/// a non-zero exit code.
///
/// The provided function is passed a [`task::CancelSignal`] that is triggered
/// when the process receives a termination signal (SIGINT, SIGTERM, or
/// SIGQUIT).
pub fn run_with<E, F>(func: impl FnOnce(task::CancelSignal) -> F + Send + 'static) -> !
where
  E: Display + Send + 'static,
  F: Future<Output = Result<(), E>> + Send + 'static,
{
  let canceler = task::Canceler::new();
  let cancel = canceler.signal();

  let mut signals = Signals::new(TERM_SIGNALS).expect("Failed to register signal handler");

  thread::start("af_core::run canceler", move || {
    for _ in &mut signals {
      break;
    }

    warn!("The process received a termination signal. Canceling the main task…");

    canceler.cancel();
  });

  run(async { func(cancel).await })
}
