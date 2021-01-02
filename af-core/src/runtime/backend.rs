use crate::prelude::*;

cfg_if! {
  if #[cfg(feature = "rt-tokio")] {
    mod tokio_0_2;

    pub use self::tokio_0_2::*;
  } else if #[cfg(feature = "rt")] {
    mod async_executor;

    pub use self::async_executor::*;
  }
}
