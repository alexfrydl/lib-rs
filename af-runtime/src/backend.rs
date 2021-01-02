#[cfg(not(feature = "tokio-0-2"))]
mod async_executor;

#[cfg(feature = "tokio-0-2")]
mod tokio_0_2;

#[cfg(not(feature = "tokio-0-2"))]
pub use self::async_executor::*;

#[cfg(feature = "tokio-0-2")]
pub use self::tokio_0_2::*;
