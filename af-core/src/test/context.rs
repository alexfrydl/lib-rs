// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Output, OutputStream, Path};
use crate::prelude::*;
use crate::string::SharedString;
use crate::{channel, task};
use std::collections::BTreeMap;

/// A test context that groups related tests together.
#[derive(Default)]
pub struct Context {
  children: BTreeMap<SharedString, Child>,
  len: usize,
  path: Path,
}

/// A child of a [`Context`].
enum Child {
  Context(Context),
  Test(Box<dyn FnOnce() -> task::Handle<Result<(), fail::Error>> + Send>),
}

impl Context {
  /// Creates a new, empty test context.
  pub fn new() -> Self {
    default()
  }

  /// Creates a new child context with a given scope name.
  pub fn scope(&mut self, name: impl Into<SharedString>, build: impl FnOnce(&mut Context)) {
    let name = name.into();

    assert!(!name.is_empty(), "A test context cannot be named \"\".");
    assert!(!self.children.contains_key(&name), "Duplicate name {:?}.", name);

    let mut ctx = Self::new();

    build(&mut ctx);

    ctx.path = self.path.clone();
    ctx.path.components.push_back(name.clone());

    self.len += ctx.len;
    self.children.insert(name, Child::Context(ctx));
  }

  /// Adds a test to the context.
  pub fn test(&mut self, name: impl Into<SharedString>, test: impl task::Future<Result>) {
    let name = name.into();

    assert!(!name.is_empty(), "A test cannot be named \"\".");
    assert!(!self.children.contains_key(&name), "Duplicate name {:?}.", name);

    let start = Box::new(move || task::start(test));

    self.len += 1;
    self.children.insert(name, Child::Test(start));
  }

  /// Starts the tests in this context, returning an [`OutputStream`] for
  /// receiving the results.
  pub fn start(self) -> OutputStream {
    let remaining = self.len;
    let (tx, rx) = channel::unbounded();
    let _task = task::start(self.run(default(), tx));

    OutputStream { remaining, rx, _task }
  }

  /// Runs the tests in this context and its child contexts in separate tasks.
  #[future::boxed]
  async fn run(self, path: Path, output: channel::Sender<Output>) {
    let mut tasks = task::Join::new();

    for (name, child) in self.children {
      let output = output.clone();
      let mut path = path.clone();

      path.components.push_back(name);

      match child {
        Child::Context(ctx) => tasks.start(ctx.run(path, output)),

        Child::Test(start) => tasks.start(async move {
          let result = start().await.map_err(fail::from).and_then(|res| res.map_err(fail::from));

          output.send(Output { path, result }).await.unwrap();
        }),
      };
    }

    while let Some(task) = tasks.next().await {
      if task.result.is_err() {
        error!("Internal test runner panic.");
      }
    }
  }
}
