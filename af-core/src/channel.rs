// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A multi-producer, multi-consumer channel.

use crate::prelude::*;

use async_channel::TryRecvError;

/// A cloneable receiver for a channel.
pub struct Receiver<T> {
  rx: async_channel::Receiver<T>,
}

/// A cloneable sender for a channel.
pub struct Sender<T> {
  tx: async_channel::Sender<T>,
}

/// An error indicating that the channel is closed.
#[derive(Clone, Copy, Debug, Default, Error)]
#[error("Channel is closed.")]
pub struct ClosedError;

/// An error returned from a [`Sender::send()`] or [`Sender::try_send()`] call.
#[derive(Clone, Copy)]
pub struct SendError<M> {
  /// The message that failed to send.
  pub msg: M,
  /// The reason for this error.
  pub reason: SendErrorReason,
}

/// The reason a [`SendError`] was returned.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum SendErrorReason {
  #[error("Channel is closed.")]
  Closed,
  #[error("Channel is full.")]
  Full,
}

/// Creates a channel with a buffer of a given capacity.
pub fn with_capacity<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
  let (tx, rx) = async_channel::bounded(capacity);

  (Sender { tx }, Receiver { rx })
}

/// Creates an channel whose buffer can grow unbounded.
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
  let (tx, rx) = async_channel::unbounded();

  (Sender { tx }, Receiver { rx })
}

impl<T> Receiver<T> {
  /// Returns `true` if the channel is closed.
  ///
  /// The channel is closed if all [`Sender`] clones are dropped.
  pub fn is_closed(&self) -> bool {
    self.rx.is_closed()
  }

  /// Waits for an available message in the channel and receives it.
  pub async fn recv(&self) -> Result<T, ClosedError> {
    self.rx.recv().await.map_err(|_| ClosedError)
  }

  /// Attempts to immediately receive an available message from the channel.
  ///
  /// If the channel is empty, this function returns `None`.
  pub fn try_recv(&self) -> Result<Option<T>, ClosedError> {
    match self.rx.try_recv() {
      Ok(msg) => Ok(Some(msg)),
      Err(TryRecvError::Empty) => Ok(None),
      Err(TryRecvError::Closed) => Err(ClosedError),
    }
  }
}

impl<T> Sender<T> {
  /// Returns `true` if the channel is closed.
  ///
  /// The channel is closed if all [`Receiver`] clones are dropped.
  pub fn is_closed(&self) -> bool {
    self.tx.is_closed()
  }

  /// Waits for available space in the channel and then sends a message.
  ///
  /// If the channel is closed before the message can be sent, this function
  /// returns a [`SendError`] containing the failed message.
  pub async fn send(&self, message: T) -> Result<(), SendError<T>> {
    self
      .tx
      .send(message)
      .await
      .map_err(|err| SendError { msg: err.0, reason: SendErrorReason::Closed })
  }

  /// Attempts to send a message to the channel immediately.
  ///
  /// If the channel is closed or full, this function returns a [`SendError`]
  /// containing the failed message.
  pub fn try_send(&self, message: T) -> Result<(), SendError<T>> {
    self.tx.try_send(message).map_err(|err| match err {
      async_channel::TrySendError::Full(msg) => SendError { msg, reason: SendErrorReason::Full },
      async_channel::TrySendError::Closed(msg) => {
        SendError { msg, reason: SendErrorReason::Closed }
      }
    })
  }
}

// Manually implement `Clone` for all `T`.

impl<T> Clone for Receiver<T> {
  fn clone(&self) -> Self {
    Self { rx: self.rx.clone() }
  }
}

impl<T> Clone for Sender<T> {
  fn clone(&self) -> Self {
    Self { tx: self.tx.clone() }
  }
}

// Implement SendError`.

impl<M> std::error::Error for SendError<M> {}

impl<M> Debug for SendError<M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Debug::fmt(&self.reason, f)
  }
}

impl<M> Display for SendError<M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.reason, f)
  }
}
