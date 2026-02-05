use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum FileType {
  Rust,
  #[default]
  Text,
}

impl Display for FileType {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
    match self {
      Self::Rust => write!(formatter, "Rust"),
      Self::Text => write!(formatter, "Text"),
    }
  }
}
