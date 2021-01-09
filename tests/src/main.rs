use af_core::prelude::*;
use af_core::task;

#[af_core::main]
pub async fn main() -> Result {
  let bg = task::start(async {
    task::sleep(Duration::secs(1)).await;

    panic!("lol");
  });

  bg.await?;

  Ok(())
}
