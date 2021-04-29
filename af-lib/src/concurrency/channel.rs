// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A multi-producer, multi-consumer channel.

use crate::prelude::*;

/// Creates a bounded channel and returns its [`BoundedSender`] and
/// [`Receiver`] halves.
///
/// A bounded channel has a fixed capacity and senders must wait for available
/// space to send messages. A channel with zero capacity is a “rendesvouz
/// channel,” where every `send` must be paired with a concurrent `recv`.
pub fn bounded<T>(capacity: usize) -> (BoundedSender<T>, Receiver<T>) {
  let (tx, rx) = flume::bounded(capacity);

  (BoundedSender(tx), Receiver(rx))
}

/// Creates an unbounded channel and returns its [`Sender`] and [`Receiver`]
/// halves.
///
/// An unbounded channel stores unlimited messages and senders can always send a
/// message without waiting.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
  let (tx, rx) = flume::unbounded();

  (Sender(tx), Receiver(rx))
}

/// A cloneable sender for a bounded channel.
pub struct BoundedSender<T>(flume::Sender<T>);

impl<T> BoundedSender<T> {
  /// Returns `true` if the channel is closed.
  ///
  /// The channel is closed if all [`Receiver`] clones are dropped.
  pub fn is_closed(&self) -> bool {
    self.0.is_disconnected()
  }

  /// Returns `true` if the channel has no messages.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Returns `true` if the channel has no remaining capacity for messages.
  pub fn is_full(&self) -> bool {
    self.0.is_full()
  }

  /// Returns the number of messages in the channel.
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Waits for available capacity in the channel, then sends a message.
  ///
  /// This function returns `true` if the message was sent or `false` if the
  /// channel is closed.
  pub async fn send(&self, message: T) -> bool {
    self.try_send(message).await.is_ok()
  }

  /// Attempts to send a message on the channel.
  ///
  /// If the channel is closed, this function returns an error containing the
  /// failed message.
  pub async fn try_send(&self, message: T) -> Result<(), MessageError<T, Closed>> {
    self.0.send_async(message).await.map_err(|err| MessageError { message: err.0, error: Closed })
  }

  /// Sends a message on the channel immediately.
  ///
  /// This function returns `true` if the message was sent or `false` if the
  /// channel is full or closed.
  pub fn send_now(&self, message: T) -> bool {
    self.0.try_send(message).is_ok()
  }

  /// Attempts to send a message on the channel immediately.
  ///
  /// If the channel is full or closed, this function returns an error
  /// containing the failed message.
  pub fn try_send_now(&self, message: T) -> Result<(), MessageError<T, SendNowError>> {
    self.0.try_send(message).map_err(|err| match err {
      flume::TrySendError::Disconnected(message) => {
        MessageError { message, error: SendNowError::Closed }
      }

      flume::TrySendError::Full(message) => MessageError { message, error: SendNowError::Full },
    })
  }
}

impl<T> Clone for BoundedSender<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> From<Sender<T>> for BoundedSender<T> {
  fn from(sender: Sender<T>) -> Self {
    BoundedSender(sender.0)
  }
}

/// An error indicating that the channel is closed.
#[derive(Clone, Copy, Debug, Default, Error)]
#[error("channel is closed")]
pub struct Closed;

/// An error containing a message that failed to send.
pub struct MessageError<M, E> {
  /// The message that failed to send.
  pub message: M,
  /// The error that caused the send to fail.
  pub error: E,
}

impl<M, E> Debug for MessageError<M, E>
where
  E: Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.error.fmt(f)
  }
}

impl<M, E> Display for MessageError<M, E>
where
  E: Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.error.fmt(f)
  }
}

impl<M, E> Error for MessageError<M, E> where E: Debug + Display {}

/// A cloneable receiver for a channel.
pub struct Receiver<T>(flume::Receiver<T>);

impl<T> Receiver<T> {
  /// Returns `true` if the channel is closed.
  ///
  /// The channel is closed if all [`Sender`] clones are dropped.
  pub fn is_closed(&self) -> bool {
    self.0.is_disconnected()
  }

  /// Returns `true` if no messages are queued on the channel.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Returns the number of messages queued in the channel.
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Waits for an available message in the channel and receives it.
  ///
  /// If the channel is closed, this function returns `None`.
  pub async fn recv(&self) -> Option<T> {
    self.try_recv().await.ok()
  }

  /// Immediately receives a message from the channel if one is available.
  pub fn recv_now(&self) -> Option<T> {
    self.try_recv_now().ok()?
  }

  /// Attempts to wait for an available message in the channel and receive it.
  ///
  /// If the channel is closed, this function returns an error.
  pub async fn try_recv(&self) -> Result<T, Closed> {
    self.0.recv_async().await.map_err(|_| Closed)
  }

  /// Attempts to immediately receive a message from the channel if one is
  /// available.
  ///
  /// If the channel is closed, this function returns an error.
  pub fn try_recv_now(&self) -> Result<Option<T>, Closed> {
    match self.0.try_recv() {
      Ok(msg) => Ok(Some(msg)),
      Err(flume::TryRecvError::Empty) => Ok(None),
      Err(flume::TryRecvError::Disconnected) => Err(Closed),
    }
  }
}

impl<T> Clone for Receiver<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

/// A cloneable sender for an unbounded channel.
pub struct Sender<T>(flume::Sender<T>);

impl<T> Sender<T> {
  /// Returns `true` if the channel is closed.
  ///
  /// The channel is closed if all [`Receiver`] clones are dropped.
  pub fn is_closed(&self) -> bool {
    self.0.is_disconnected()
  }

  /// Returns `true` if no messages are queued on the channel.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Returns the number of messages queued in the channel.
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Sends a message on the channel.
  ///
  /// This function returns `true` if the message was sent or `false` if the
  /// channel is closed.
  pub fn send(&self, message: T) -> bool {
    self.0.try_send(message).is_ok()
  }

  /// Attempts to send a message on the channel.
  ///
  /// If the channel is closed, this function returns an error containing the
  /// failed message.
  pub fn try_send(&self, message: T) -> Result<(), MessageError<T, Closed>> {
    self.0.try_send(message).map_err(|err| MessageError {
      message: match err {
        flume::TrySendError::Disconnected(msg) => msg,
        flume::TrySendError::Full(msg) => msg,
      },
      error: Closed,
    })
  }
}

impl<T> Clone for Sender<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

/// An error returned from an immediate send attempt.
#[derive(Debug, Error)]
pub enum SendNowError {
  /// Channel is closed.
  #[error("channel is closed")]
  Closed,
  /// Channel is full.
  #[error("channel is full")]
  Full,
}
