use af_core::prelude::*;
use af_core::task;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
  A,
  B(bool),
}

#[af_core::main]
pub async fn main() -> Result {
  use af_core::sync::AtomicCell;

  let cell = Arc::new(AtomicCell::new(State::A));
  let mut listener = cell.listen();

  let bg = task::start(async move {
    task::sleep(Duration::secs(1)).await;

    cell.store(State::B(true));
  });

  println!("Before: {:?}", listener.next().await);
  println!("After: {:?}", listener.await);

  bg.join().await?;

  Ok(())
}
