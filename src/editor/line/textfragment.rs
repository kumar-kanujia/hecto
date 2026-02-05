use crate::{editor::line::graphemewidth::GraphemeWidth, prelude::ByteIdx};

#[derive(Debug, Clone)]
pub struct TextFragment {
  pub grapheme: String,
  pub rendered_width: GraphemeWidth,
  pub replacement: Option<char>,
  pub start: ByteIdx,
}
