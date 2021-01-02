/// Runs a block of code and catches `?` operator returns.
///
/// This is just a cleaner version of creating a closure and immediately calling
/// it. It is intended to serve as a replacement for `try { .. }` until it is
/// stable.
#[macro_export]
macro_rules! attempt {
  ($($tokens:tt)+) => {
    (|| { $($tokens)+ })()
  };
}

/// Runs a block of async code and catches `?` operator returns.
///
/// This is just a cleaner version of creating a closure and immediately calling
/// it. It is intended to serve as a replacement for `try { .. }` until it is
/// stable.
#[macro_export]
macro_rules! attempt_async {
  ($($tokens:tt)+) => {
    (|| async { $($tokens)+ })().await
  };
}
