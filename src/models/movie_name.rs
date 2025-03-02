use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct MovieName(String);

impl MovieName {
  pub fn new(value: &str) -> Self {
    Self(value.to_string())
  }

  pub fn name(&self) -> String {
    self.0.clone()
  }
}

impl fmt::Display for MovieName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}
