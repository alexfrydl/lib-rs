// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Common utilities for interacting with panics.

pub use std::panic::{AssertUnwindSafe, RefUnwindSafe, UnwindSafe};

use crate::prelude::*;

/// A captured panic.
#[derive(Error, Debug, From)]
pub struct Panic {
  /// The file in which the panic occurred.
  pub file: String,
  /// The line on which the panic occurred.
  pub line: usize,
  /// The panic message, if one was provided.
  pub message: Option<Cow<'static, str>>,
}

/// Thread-specific state for the panic hook.
#[derive(Default)]
struct HookState {
  capture: bool,
  location: Option<(String, usize)>,
}

thread_local! {
  /// Thread-specific state for the panic hook.
  static HOOK_STATE: RefCell<HookState> = default();
}

/// Run a closure, capturing information about a panic if one occurs.
pub fn capture<O>(closure: impl FnOnce() -> O + UnwindSafe) -> Result<O, Panic> {
  // Ensure the panic hook is installed.
  install_hook();

  // Run the closure, capturing location information and catching the panic
  // value.

  HOOK_STATE.with(|state| state.borrow_mut().capture = true);

  let result = std::panic::catch_unwind(closure);

  HOOK_STATE.with(|state| state.borrow_mut().capture = false);

  // Map the result to use Panic error type.

  result.map_err(|value| {
    // Convert the panic value into a string message if possible.

    let message = match value.downcast::<&'static str>() {
      Ok(message) => Some((*message).into()),

      Err(value) => match value.downcast::<String>() {
        Ok(message) => Some((*message).into()),

        Err(_) => None,
      },
    };

    // Extract the location information captured by the hook.

    let (file, line) = HOOK_STATE
      .with(|state| state.borrow_mut().location.take())
      .unwrap_or_else(|| ("<unknown>".to_string(), 0));

    // Return the custom Panic error.

    Panic { file, line, message }
  })
}

/// Install a global panic hook for capturing location information.
///
/// This function only runs the first time it is called.
fn install_hook() {
  use std::sync::Once;

  static INSTALL: Once = Once::new();

  INSTALL.call_once(|| {
    let original = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
      HOOK_STATE.with(|state| {
        let state = &mut state.borrow_mut();

        if state.capture {
          state.location = info.location().map(|loc| (loc.file().to_string(), loc.line() as usize));
        } else {
          original(info);
        }
      });
    }))
  })
}

// Implement formatting.

impl Display for Panic {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "panicked at {} line {}", self.file, self.line)?;

    if let Some(message) = &self.message {
      write!(f, "\n  {}", fmt::indent("", "  ", message))?;
    }

    Ok(())
  }
}
