use crate::prelude::*;

cfg_if! {
  if #[cfg(feature = "runtime-tokio")] {
    mod tokio_0_2;

    pub use self::tokio_0_2::*;
  } else if #[cfg(feature = "runtime")] {
    mod async_executor;

    pub use self::async_executor::*;
  } else {
    mod none;

    pub use self::none::*;
  }
}
