use super::*;
use crate::fs::path;

/// A wrapper returned from [`as_path()`] that displays a string as a filesystem
/// path.
pub struct AsPath<'a>(&'a str);

/// Wraps a string so that it displays as a filesystem path.
pub fn as_path(path: &str) -> AsPath {
  AsPath(path)
}

impl Debug for AsPath<'_> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", path::as_std(self.0))
  }
}

impl Display for AsPath<'_> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", path::as_std(self.0).display())
  }
}
