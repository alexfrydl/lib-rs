// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Synchronization primitives and concurrency utilties.

pub mod atomic_cell;
pub mod channel;
pub mod semaphore;

pub use self::atomic_cell::AtomicCell;
pub use self::channel::unbounded as channel;
pub use self::semaphore::Semaphore;
