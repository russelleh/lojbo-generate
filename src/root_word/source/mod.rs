use std::fmt;

#[derive(Eq, Hash, PartialEq)]
pub struct Source {
  pub value:  String,
}

impl Source {
  pub fn new(value: String) -> Source {
    Source {
      value
    }
  }
}

impl fmt::Display for Source {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}
