// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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

  ($cx:expr, $name:expr, repeat = $times:literal, $($rest:tt)+) => {
    $cx.test($name, async move {
      for _ in 0..$times {
        $($rest)+;

        af_core::task::yield_now().await;
      }

      #[allow(unreachable_code)]
      Ok(())
    })
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
