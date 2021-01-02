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
#[derive(Clone, Copy, Debug, Default, Display, Error)]
#[display(fmt = "Channel is closed.")]
pub struct ClosedError;

/// An error that occurred during a [`Channel::send()`] or
/// [`Channel::try_send()`] call.
#[derive(Clone, Copy)]
pub struct SendError<M> {
  /// The message that failed to send.
  pub msg: M,
  /// The reason for this error.
  pub reason: SendErrorReason,
}

/// One of the possible kinds of [`SendError`].
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum SendErrorReason {
  #[display(fmt = "Channel is closed.")]
  Closed,
  #[display(fmt = "Channel is full.")]
  Full,
}

/// Creates a bounded channel with a specified capacity.
///
/// Bounded channels can only buffer up to `capacity` unreceived messages. If
/// the channels is full, [`Sender::send()`] will wait for space.
pub fn bounded<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
  let (tx, rx) = async_channel::bounded(capacity);

  (Sender { tx }, Receiver { rx })
}

/// Creates an unbounded channel.
///
/// Unbounded channels can buffer an unlimited number of unreceived messages,
/// and [`Sender::send()`] will never wait.
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

  /// Sends a message to the channel immediately.
  ///
  /// If the channel is closed or full, this function returns `false`. Use
  /// [`try_send()`] for extended error information.
  pub fn send(&self, message: T) -> bool {
    self.try_send(message).is_ok()
  }

  /// Waits for available space in the channel and then sends a message.
  ///
  /// If the channel is closed before the message can be sent, this function
  /// returns `false`. Use [`try_send_queued()`] for extended error information.
  pub async fn send_queued(&self, message: T) -> Result<(), SendError<T>> {
    self
      .tx
      .send(message)
      .await
      .map_err(|err| SendError { msg: err.0, reason: SendErrorReason::Closed })
  }

  /// Attempts to send a message to the channel immediately.
  ///
  /// If the channel is closed or full, this function returns a[`SendError`]
  /// containing the failed message.
  pub fn try_send(&self, message: T) -> Result<(), SendError<T>> {
    self.tx.try_send(message).map_err(|err| match err {
      async_channel::TrySendError::Full(msg) => SendError { msg, reason: SendErrorReason::Full },
      async_channel::TrySendError::Closed(msg) => {
        SendError { msg, reason: SendErrorReason::Closed }
      }
    })
  }

  /// Attempts to wait for available space in the channel and then send a
  /// message.
  ///
  /// If the channel is closed before the message can be sent, this function
  /// returns a [`SendError`] containing the failed message.
  pub async fn try_send_queued(&self, message: T) -> Result<(), SendError<T>> {
    self
      .tx
      .send(message)
      .await
      .map_err(|err| SendError { msg: err.0, reason: SendErrorReason::Closed })
  }
}

// Implement `Stream` for the receiver end.

impl<T> Stream for Receiver<T> {
  type Item = T;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<Option<T>> {
    Pin::new(&mut self.rx).poll_next(cx)
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
