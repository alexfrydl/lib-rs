use af_core::prelude::*;
use af_core::task;
use std::sync::atomic::{AtomicUsize, Ordering::AcqRel};

#[af_core::main]
pub async fn main() -> Result {
  let mut tasks = task::Parallel::<_, Infallible>::new();
  let value: Arc<AtomicUsize> = default();

  for _ in 0..100 {
    let value = value.clone();

    tasks.add(async move {
      if random::ratio(1, 20) {
        panic!("Critical miss!");
      }

      Ok(value.fetch_add(1, AcqRel))
    });
  }

  while let Some(task) = tasks.next().await {
    match task.output {
      Ok(value) if task.index != value => {
        info!("Task {} returned {}.", task.index, value);
      }

      Err(err) => {
        error!("Task {} failed. {}", task.index, err);
      }

      _ => {}
    }
  }

  Ok(())
}
