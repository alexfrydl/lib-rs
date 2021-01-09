use af_core::prelude::*;
use af_core::task;

#[af_core::main]
pub async fn main(cancel_signal: task::CancelSignal) {
  future::race(task::sleep(Duration::secs(5)), cancel_signal).await;
}
