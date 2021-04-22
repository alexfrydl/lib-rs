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

/// An error returned from a [`Sender::try_send()`] call.
#[derive(Clone, Copy)]
pub struct SendError<M>(pub M);

/// Creates an asynchronous channel.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
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

  /// Returns `true` if no messages are queued on the channel.
  pub fn is_empty(&self) -> bool {
    self.rx.is_empty()
  }

  /// Returns the number of messages queued in the channel.
  pub fn len(&self) -> usize {
    self.rx.len()
  }

  /// Waits for an available message in the channel and receives it.
  ///
  /// If the channel is closed, this function returns `None`.
  pub async fn recv(&self) -> Option<T> {
    self.try_recv().await.ok()
  }

  /// Immediately receives an available message from the channel.
  ///
  /// If the channel is empty or closed, this function returns `None`.
  pub fn recv_now(&self) -> Option<T> {
    self.try_recv_now().ok()?
  }

  /// Attempts to wait for an available message in the channel and receive it.
  ///
  /// If the channel is closed, this function returns an error.
  pub async fn try_recv(&self) -> Result<T, ClosedError> {
    self.rx.recv().await.map_err(|_| ClosedError)
  }

  /// Attempts to immediately receive an available message from the channel.
  ///
  /// If the channel is empty, this function returns `Ok(None)`. If the channel is closed, this
  /// function returns an error
  pub fn try_recv_now(&self) -> Result<Option<T>, ClosedError> {
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

  /// Returns `true` if no messages are queued on the channel.
  pub fn is_empty(&self) -> bool {
    self.tx.is_empty()
  }

  /// Returns the number of messages queued in the channel.
  pub fn len(&self) -> usize {
    self.tx.len()
  }

  /// Sends a message on the channel.
  ///
  /// If the channel is closed, this function returns a `Some(T)` containing the
  /// failed message.
  pub fn send(&self, message: T) -> Option<T> {
    match self.try_send(message) {
      Ok(_) => None,
      Err(err) => Some(err.0),
    }
  }

  /// Attempts to send a message on the channel.
  ///
  /// If the channel is closed, this function returns a [`SendError`] containing
  /// the failed message.
  pub fn try_send(&self, message: T) -> Result<(), SendError<T>> {
    self.tx.try_send(message).map_err(|err| SendError(err.into_inner()))
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

// Implement SendError error functionality.

impl<M> std::error::Error for SendError<M> {}

impl<M> Debug for SendError<M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", ClosedError)
  }
}

impl<M> Display for SendError<M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", ClosedError)
  }
}
