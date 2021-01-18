#[macro_export]
macro_rules! test {

  ($cx:expr, $name:expr, timeout = immediate, $($rest:tt)+) => {
    $cx.test(
      $name,
      af_core::future::race(
        async move {
          $($rest)+;

          #[allow(unreachable_code)]
          Ok(())
        },
        async { fail!("Timed out.") },
      ),
    )
  };

  ($cx:expr, $name:expr, timeout = $timeout:literal, $($rest:tt)+) => {
    $cx.test(
      $name,
      af_core::future::race(
        async move {
          $($rest)+;

          #[allow(unreachable_code)]
          Ok(())
        },
        async move {
          af_core::task::sleep($timeout.parse().expect("Failed to parse timeout")).await;
          fail!("Timed out.")
        },
      ),
    )
  };

  ($cx:expr, $name:expr, $($rest:tt)+) => {
    $cx.test($name, async move {
      $($rest)+;

      #[allow(unreachable_code)]
      Ok(())
    })
  };

}
