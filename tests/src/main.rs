use af_core::prelude::*;
use af_core::task;

#[af_core::main]
pub async fn main(cancel: task::CancelSignal) -> Result {
  let mut batch = task::Batch::new();
  let canceler = task::Canceler::inherit(cancel);

  for i in 1..=10 {
    batch.add(task::start(test(canceler.signal()))).with_name(format!("Cool task {}", i));
  }

  batch.set_canceler(canceler);
  batch.run().await?;

  Ok(())
}

async fn test(cancel: task::CancelSignal) -> Result<bool> {
  let sleep = task::sleep(Duration::secs(random::range(1.0..5.0)));

  if cancel.guard(sleep).await.is_err() {
    return Ok(false);
  }

  match random::<u64>() % 3 {
    0 => Ok(true),
    1 => panic!("Oh no"),
    _ => fail!("Woops."),
  }
}
