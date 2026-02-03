use crate::editor::line::graphemewidth::GraphemeWidth;

#[derive(Debug, Clone)]
pub struct TextFragment {
  pub grapheme: String,
  pub rendered_width: GraphemeWidth,
  pub replacement: Option<char>,
  pub start_byte_idx: usize,
}
