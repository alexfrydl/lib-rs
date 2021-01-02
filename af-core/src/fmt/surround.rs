use super::*;

/// A wrapper returned from [`surround()`] that displays a value surrounded by
/// a custom prefix and suffix.
pub struct Surrounded<'a, T>(&'a str, T, &'a str);

/// Wraps a value so that it displays with the given prefix and suffix strings.
pub fn surround<'a, T>(prefix: &'a str, value: T, suffix: &'a str) -> Surrounded<'a, T> {
  Surrounded(prefix, value, suffix)
}

impl<T: Debug> Debug for Surrounded<'_, T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)?;
    self.1.fmt(f)?;
    write!(f, "{}", self.2)
  }
}

impl<T: Display> Display for Surrounded<'_, T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)?;
    self.1.fmt(f)?;
    write!(f, "{}", self.2)
  }
}
