use crate::editor::annotatedstring::{AnnotatedString, annotatedstringpart::AnnotatedStringPart};

use std::cmp::min;

/// Iterator returned by AnnotatedString
pub struct AnnotatedStringIterator<'a> {
  /// Reference to the AnnotatedString with an liftime valid till the end of the iterator
  pub annotated_string: &'a AnnotatedString,
  /// Keep track of current byte index
  pub current_idx: usize,
}

// Implimentation of Iterator trait
impl<'a> Iterator for AnnotatedStringIterator<'a> {
  // Return type of the iterator
  type Item = AnnotatedStringPart<'a>;

  // Return the next item in the iterator
  fn next(&mut self) -> Option<Self::Item> {
    // Exit if current index is beyond the end of the string
    // Then there is nothing to return
    if self.current_idx >= self.annotated_string.string.len() {
      return None;
    }

    // Find the current active annotation
    if let Some(annotation) = self
      .annotated_string
      .annotation
      .iter()
      // Filter out all annotation that does not return true
      // It filter out active annotation
      // ___ |_↓_| ___
      .filter(|annotation| {
        annotation.start_byte_idx <= self.current_idx && annotation.end_byte_idx > self.current_idx
      })
      // In case of mutiple take last
      .last()
    {
      // Start and end of the string slice at which annonation ends
      let start_idx = self.current_idx;
      let end_idx = min(annotation.end_byte_idx, self.annotated_string.string.len());

      // Advance the current index
      self.current_idx = end_idx;

      // This is the annotated part of string
      return Some(AnnotatedStringPart {
        string: &self.annotated_string.string[start_idx..end_idx],
        annotation_type: Some(annotation.annotation_type),
      });
    }

    // Find the boundry of the nearest annotation to the right
    // If not found then end will be end of the string
    let mut end_idx = self.annotated_string.string.len();
    for annotation in &self.annotated_string.annotation {
      if annotation.start_byte_idx > self.current_idx && annotation.start_byte_idx < end_idx {
        end_idx = annotation.start_byte_idx;
      }
    }

    let start_idx = self.current_idx;
    self.current_idx = end_idx;

    // Return the part with no active annotation
    Some(AnnotatedStringPart {
      string: &self.annotated_string.string[start_idx..end_idx],
      annotation_type: None,
    })
  }
}
