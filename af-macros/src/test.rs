#[macro_export]
macro_rules! test {
  ($cx:expr, $name:expr, $block:expr) => {
    $cx.test($name, async move {
      $block;

      #[allow(unreachable_code)]
      Ok(())
    })
  };
}
