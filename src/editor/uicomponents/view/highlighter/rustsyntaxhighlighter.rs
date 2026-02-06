use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

use crate::{
  editor::{
    annotation::Annotation, annotationtype::AnnotationType, line::Line,
    uicomponents::view::highlighter::syntaxhighlighter::SyntaxHighlighter,
  },
  prelude::LineIdx,
};

#[derive(Default)]
pub struct RustSyntaxHighlighter {
  highlights: HashMap<LineIdx, Vec<Annotation>>,
}

fn is_valid_number(word: &str) -> bool {
  if word.is_empty() {
    return false;
  }

  if is_numeric_literal(word) {
    return true;
  }

  let mut chars = word.chars();

  // Check the first character
  if let Some(first_char) = chars.next()
    && !first_char.is_ascii_digit()
  // Numbers must start with a digit
  {
    return false;
  }

  let mut seen_dot = false;
  let mut seen_e = false;
  let mut prev_was_digit = true;

  // Iterate over the remaining characters
  for char in chars {
    match char {
      '0'..='9' => {
        prev_was_digit = true;
      }
      '_' => {
        // Underscores must be between digits
        if !prev_was_digit {
          return false;
        }
        prev_was_digit = false;
      }
      '.' => {
        // Disallow multiple dots, dots after 'e' or dots not after a digit
        if seen_dot || seen_e || !prev_was_digit {
          return false;
        }
        seen_dot = true;
        prev_was_digit = false;
      }
      'e' | 'E' => {
        // Disallow multiple 'e's or 'e' not after a digit
        if seen_e || !prev_was_digit {
          return false;
        }
        seen_e = true;
        prev_was_digit = false;
      }
      _ => {
        // Invalid character
        return false;
      }
    }
  }

  // Must end with a digit
  prev_was_digit
}

fn is_numeric_literal(word: &str) -> bool {
  if word.len() < 3 {
    //For a literal, we need a leading `0`, a suffix and at least one digit
    return false;
  }

  let mut chars = word.chars();
  if chars.next() != Some('0') {
    // Check the first character for a leading 0
    return false;
  }
  let base = match chars.next() {
    //Check the second character for a proper base
    Some('b' | 'B') => 2,
    Some('o' | 'O') => 8,
    Some('x' | 'X') => 16,
    _ => return false,
  };
  chars.all(|char| char.is_digit(base))
}

impl RustSyntaxHighlighter {
  fn highlight_digits(line: &Line, result: &mut Vec<Annotation>) {
    line.split_word_bound_indices().for_each(|(start, word)| {
      if is_valid_number(word) {
        result.push(Annotation {
          annotation_type: AnnotationType::Number,
          start,
          end: start.saturating_add(word.len()),
        });
      }
    });
  }
}

impl SyntaxHighlighter for RustSyntaxHighlighter {
  fn highlight(&mut self, idx: LineIdx, line: &Line) {
    let mut result = Vec::new();
    Self::highlight_digits(line, &mut result);
    self.highlights.insert(idx, result);
  }

  fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
    self.highlights.get(&idx)
  }
}
