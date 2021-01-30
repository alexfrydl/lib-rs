// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::channel;
use crate::prelude::*;
use std::sync::atomic::{self, AtomicU64};

struct Id {
  index: u64,
  thread: u64,
}

enum ExitReason {
  Ok,
  Err(String),
  Panic(Panic),
}

pub fn start<F>(func: impl FnOnce() -> F + Send + 'static) -> Id
where
  F: Future + Send + 'static,
  F::Output: IntoResult + Send + 'static,
{
  static NEXT_THREAD_ID: AtomicU64 = AtomicU64::new(0);

  thread_local! {
    static THREAD_ID: u64 = NEXT_THREAD_ID.fetch_add(1, atomic::Ordering::AcqRel);
    static TASK_ID: RefCell<u64> = RefCell::new(0);
  };

  tokio::task_local! {
    static TASK_TX: channel::Sender<tokio::task::JoinHandle<ExitReason>>;
  };

  let thread_id = THREAD_ID.with(|id| *id);

  let id = TASK_ID.with(|cell| {
    let id = cell.borrow_mut();
    *id += 1;
    *id
  });

  let task = tokio::spawn(async move {
    match func().await.into_result() {
      Ok(()) => ExitReason::Ok,
      Err(err) => ExitReason::Err(err.to_string()),
    }
  });

  TASK_TX.with(|tx| tx.try_send(task).unwrap());
}

pub trait IntoResult {
  type Err: ToString;

  fn into_result(self) -> Result<(), Self::Err>;
}

impl<T, E> IntoResult for Result<T, E>
where
  E: ToString,
{
  type Err = E;

  fn into_result(self) -> Result<(), E> {
    match self {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    }
  }
}

impl IntoResult for () {
  type Err = Infallible;

  fn into_result(self) -> Result<(), Self::Err> {
    Ok(())
  }
}
