//! Simple timeouts for async operations.

use super::Duration;
use crate::prelude::*;
use crate::util::future;

/// Waits for an async operation to complete with a timeout.
///
/// If the timeout duration elapses before the operation completes, this
/// function returns an error and drops the operation.
pub async fn timeout<O>(duration: Duration, op: impl Future<Output = O>) -> Result<O, Error> {
  let future = async { Ok(op.await) };

  if duration.is_infinite() {
    return future.await;
  }

  let timeout = async {
    duration.elapsed().await;
    Err(Error)
  };

  future::race(future, timeout).await
}

/// A timeout error.
#[derive(Debug, Error)]
#[error("timed out")]
pub struct Error;
