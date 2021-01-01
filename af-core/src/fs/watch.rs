// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A file system watcher.

use crate::fs;
use crate::prelude::*;
use crate::sync::channel;
use crate::thread;
use notify::Watcher as _;
use std::io;
use std::sync::mpsc;

/// One of the possible [`Watcher`] events.
#[derive(Debug)]
pub enum Event {
  /// A new file or directory was created.
  Created(String),
  /// A file was modified.
  Modified(String),
  /// A file was removed.
  Removed(String),
  /// A file was renamed.
  Renamed { from: String, to: String },
}

/// A file system watcher that can watch paths for events.
pub struct Watcher {
  inner: notify::RecommendedWatcher,
}

/// Processes debounced events received from a `notify::Watcher` and sends the
/// mapped events to the given channel.
///
/// This function regularly blocks the current thread.
async fn map_inner_events(
  inner_events: mpsc::Receiver<notify::DebouncedEvent>,
  events: channel::Sender<Event>,
) -> Result {
  trace!("Waiting for events…");

  for inner_event in inner_events {
    trace!("Received event `{:?}`.", inner_event);

    match inner_event {
      notify::DebouncedEvent::Create(path) => {
        if let Some(path) = path.to_str() {
          events.send(Event::Created(path.into())).await?;
        }
      }

      notify::DebouncedEvent::Write(path) => {
        if let Some(path) = path.to_str() {
          events.send(Event::Modified(path.into())).await?;
        }
      }

      notify::DebouncedEvent::Remove(path) => {
        if let Some(path) = path.to_str() {
          events.send(Event::Removed(path.into())).await?;
        }
      }

      notify::DebouncedEvent::Rename(from, to) => {
        if let (Some(from), Some(to)) = (from.to_str(), to.to_str()) {
          events.send(Event::Renamed { from: from.into(), to: to.into() }).await?;
        }
      }

      notify::DebouncedEvent::Error(err, Some(path)) => {
        error!("(at `{}`) {}", path.display(), err);
      }

      notify::DebouncedEvent::Error(err, None) => {
        error!("{}", err);
      }

      _ => {}
    }
  }

  Ok(())
}

impl Watcher {
  /// Creates a new file system watcher that sends events to the given channel.
  pub fn new(events_tx: channel::Sender<Event>) -> Result<Self> {
    let (inner_events_tx, inner_events) = mpsc::channel();
    let inner = notify::Watcher::new(inner_events_tx, std::time::Duration::from_millis(100))?;

    thread::start_detached("__af_corefs::watch::Watcher", move || {
      thread::block_on(map_inner_events(inner_events, events_tx))
    });

    Ok(Self { inner })
  }

  /// Begins watching the given directory for events.
  pub fn watch_dir<'a>(&mut self, path: impl fs::PathLike<'a>) -> Result {
    let path = fs::path::join(
      fs::path::resolved(path).map_err(fail::with!("Failed to resolve directory."))?,
      "",
    );

    self.inner.watch(path.as_ref(), notify::RecursiveMode::Recursive).map_err(|err| match err {
      notify::Error::Io(err) if err.kind() == io::ErrorKind::NotFound => {
        fail::err!("Directory does not exist.")
      }
      notify::Error::Io(err) => fail::err!("IO error: {}.", err),
      notify::Error::PathNotFound => fail::err!("Directory does not exist."),
      _ => unreachable!(),
    })?;

    trace!("Watching `{}`.", path);

    Ok(())
  }

  /// Stops watching the given directory for events.
  pub fn unwatch_dir<'a>(&mut self, path: impl fs::PathLike<'a>) {
    let _ = match fs::path::resolved(path) {
      Ok(path) => self.inner.unwatch(fs::path::join(path, "").as_ref()),
      Err(_) => return,
    };
  }
}
