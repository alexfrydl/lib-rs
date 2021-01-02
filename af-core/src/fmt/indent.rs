use super::*;

/// A wrapper returned from [`indent()`] that displays its inner value with
/// custom indentation.
pub struct Indented<'a, T> {
  initial: &'a str,
  hanging: &'a str,
  value: T,
}

/// A formatter that automatically indents lines.
pub struct IndentedFormatter<'a, F> {
  f: F,
  initial: &'a str,
  hanging: &'a str,
  line: usize,
  start_of_line: bool,
}

/// Wraps the value so that it is displayed with the given initial and hanging
/// indentation.
pub fn indent<'a, T: Display>(initial: &'a str, hanging: &'a str, value: T) -> Indented<'a, T> {
  Indented { initial, hanging, value }
}

impl<'a, F: Write + 'a> IndentedFormatter<'a, F> {
  /// Creates a new indented formatter with the given initial and hanging
  /// indentation.
  pub fn new(f: F, initial: &'a str, hanging: &'a str) -> Self {
    Self { f, initial, hanging, line: 1, start_of_line: true }
  }
}

impl<'a, T: Display> Display for Indented<'a, T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let alt = f.alternate();
    let mut f = IndentedFormatter::new(f, self.initial, self.hanging);

    if alt {
      write!(f, "{:#}", self.value)
    } else {
      write!(f, "{}", self.value)
    }
  }
}

impl<'a, F: 'a + Write> Write for IndentedFormatter<'a, F> {
  fn write_str(&mut self, s: &str) -> Result {
    for c in s.chars() {
      if c == '\n' {
        self.f.write_char(c)?;
        self.start_of_line = true;
        self.line += 1;

        continue;
      }

      if self.start_of_line {
        if self.line == 1 {
          self.f.write_str(self.initial)?;
        } else {
          self.f.write_str(self.hanging)?;
        }

        self.start_of_line = false;
      }

      self.f.write_char(c)?;
    }

    Ok(())
  }
}