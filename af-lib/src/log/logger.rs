// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cell::RefCell;
use std::sync::atomic::{self, AtomicUsize};
use std::thread;

pub use af_macros::logger_init as init;
use dashmap::DashMap;
use log::{Level, LevelFilter, Log, Metadata, Record, RecordBuilder};
use parking_lot::RwLock;

use super::*;
use crate::concurrency::{channel, Channel};

/// A logger to register with the `log` crate.
struct Logger {
  dropped_messages: AtomicUsize,
  max_level: RwLock<LevelFilter>,
  max_level_of: DashMap<String, LevelFilter>,
  output: Channel<Output>,
}

/// One of the possible output commands.
enum Output {
  Flush(channel::Sender<()>),
  Write(String),
}

/// The shared logger instance.
static LOGGER: Lazy<Logger> = Lazy::new(|| Logger {
  dropped_messages: default(),
  max_level: RwLock::new(LevelFilter::Warn),
  max_level_of: default(),
  output: Channel::new(),
});

thread_local! {
  /// A thread-local buffer for formatting messages.
  static THREAD_BUFFER: RefCell<String> = default();
}

/// Initializes the logger.
pub fn init() {
  if log::set_logger(&*LOGGER).is_err() {
    return;
  }

  log::set_max_level(LevelFilter::Trace);

  set_level_of(
    "af_core",
    match cfg!(debug_assertions) {
      true => Debug,
      false => Info,
    },
  );

  thread::Builder::new()
    .name("af_runtime::logger".into())
    .spawn(|| futures_lite::future::block_on(output_messages()))
    .unwrap();
}

/// Waits until the logger finishes writing all records created before this
/// call.
pub async fn flush() {
  let (tx, rx) = channel();

  LOGGER.output.send(Output::Flush(tx));

  rx.recv().await;
}

/// Sets the level of the logger.
///
/// Records above this level are hidden. Use `None` to hide all records.
pub fn set_level(level: impl Into<Option<Level>>) {
  let level = level.into().map(|lv| lv.to_level_filter()).unwrap_or(LevelFilter::Off);
  let mut max_level = LOGGER.max_level.write();

  *max_level = level;
}

/// Sets the level of a target.
///
/// Records above this level are hidden. Use `None` to hide all records.
pub fn set_level_of(name: impl Into<String>, level: impl Into<Option<Level>>) {
  let level = level.into().map(|lv| lv.to_level_filter()).unwrap_or(LevelFilter::Off);
  let name = name.into();

  LOGGER.max_level_of.insert(name, level);
}

/// Writes each message received from the given channel to stderr.
async fn output_messages() {
  let mut buffer = String::with_capacity(128);
  let logger = &*LOGGER;
  let mut stderr = console::Term::stderr();

  loop {
    let cmd = logger.output.recv().await;

    // If one or more messages were dropped, write an error message about it.

    let dropped_messages = logger.dropped_messages.swap(0, atomic::Ordering::Relaxed);

    if dropped_messages > 0 {
      write_message(
        Time::now(),
        &RecordBuilder::new()
          .level(Level::Error)
          .target(module_path!())
          .args(format_args!(
            "Too many messages. {} {} dropped.",
            dropped_messages,
            match dropped_messages {
              1 => "message",
              _ => "messages",
            }
          ))
          .build(),
        &mut buffer,
      )
      .unwrap();

      writeln!(stderr, "{}", buffer).unwrap();

      buffer.clear();
    }

    // Then run the command.

    match cmd {
      Output::Write(message) => {
        writeln!(stderr, "{}", message).unwrap();
      }

      Output::Flush(tx) => {
        tx.send(());
      }
    }
  }
}

/// Writes a record to the given string.
fn write_message(time: Time, record: &Record, f: &mut String) -> fmt::Result {
  use console::style;

  // Write the timestamp in bright black.

  write!(f, "{} ", style(time.format("%F %T%.3f")).black().bright())?;

  // Write the log level with an appropriate color.

  match record.level() {
    Level::Trace => {
      write!(f, "{} ", style("TRACE").black().bright())?;
    }

    Level::Debug => {
      write!(f, "{} ", style("DEBUG").magenta())?;
    }

    Level::Info => {
      write!(f, " {} ", style("INFO").blue())?;
    }

    Level::Warn => {
      write!(f, " {} ", style("WARN").yellow())?;
    }

    Level::Error => {
      write!(f, "{} ", style("ERROR").red())?;
    }
  }

  // Write the source of the message.

  if !record.target().is_empty() {
    let mut name = style(fmt::surround("[", record.target(), "] "));

    name = match record.level() {
      Level::Trace => name.black().bright(),
      _ => name,
    };

    write!(f, "{}", name)?;
  }

  // Finally, write the message.

  let message = style(record.args());

  let styled = match record.level() {
    Level::Trace => message.black().bright(),
    _ => message,
  };

  write!(f, "{}", styled)
}

// Implement `Log` to send messages to the output task.

impl Log for Logger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    let mut target = Some(metadata.target());

    while let Some(t) = target {
      match self.max_level_of.get(t) {
        Some(filter) => return metadata.level() <= *filter,

        None => {
          let mut split = t.rsplitn(2, "::");

          split.next();

          target = split.next();
        }
      }
    }

    metadata.level() <= *LOGGER.max_level.read()
  }

  fn log(&self, record: &Record) {
    if !self.enabled(record.metadata()) {
      return;
    }

    let time = Time::now();

    let message = THREAD_BUFFER.with(|buffer| {
      let mut buffer = buffer.borrow_mut();

      write_message(time, record, &mut buffer).unwrap();

      buffer.split_off(0)
    });

    // Send a message to the output task immediately or increment the dropped
    // count.

    if self.output.len() > 4096 {
      LOGGER.dropped_messages.fetch_add(1, atomic::Ordering::Relaxed);
    } else {
      self.output.send(Output::Write(message));
    }
  }

  fn flush(&self) {}
}
