pub mod channel;
pub mod fiber;
pub mod runtime;
mod scope;
pub mod task;
pub mod thread;

pub use self::channel::{channel, Channel};

/// Yields once to pending concurrent operations.
pub async fn cooperative_yield() {
  futures_lite::future::yield_now().await
}

/// Waits for all children of the current concurrency scope to exit.
pub async fn join() {
  scope::current()
    .expect("join_all() must be called from within a concurrency scope")
    .join_children()
    .await
}
