pub use af_macros::{attempt, attempt_async};
pub use futures_lite::pin;

/// Returns the “default value” for a type.
pub fn default<T: Default>() -> T {
  T::default()
}
