pub mod annotatedstringiterator;
pub mod annotatedstringpart;
pub mod annotation;
pub mod annotationtype;

use crate::{
  editor::annotatedstring::{
    annotatedstringiterator::AnnotatedStringIterator, annotatedstringpart::AnnotatedStringPart,
    annotation::Annotation, annotationtype::AnnotationType,
  },
  prelude::ByteIdx,
};

use std::{
  cmp::{max, min},
  fmt::{Display, Formatter, Result},
};

/// Hold annotations, serve as a vehicle between Line to View to Terminal, where it is then finally printed out.
#[derive(Debug, Default)]
pub struct AnnotatedString {
  string: String,
  annotation: Vec<Annotation>,
}

impl AnnotatedString {
  pub fn from(string: &str) -> Self {
    Self {
      string: String::from(string),
      annotation: Vec::new(),
    }
  }

  pub fn add_annotation(&mut self, annotation_type: AnnotationType, start: ByteIdx, end: ByteIdx) {
    debug_assert!(start <= end);
    self.annotation.push(Annotation {
      annotation_type,
      start,
      end,
    });
  }

  pub fn replace(&mut self, start: usize, end: usize, new_string: &str) {
    let end = min(end, self.string.len());

    debug_assert!(start <= end);
    debug_assert!(start <= self.string.len());

    if start > end {
      return;
    }

    // We have replaced the string
    self.string.replace_range(start..end, new_string);

    // Now Modify the annotations

    // This is the range we want to replace.
    let replaced_range_len = end.saturating_sub(start);

    // Is replaced string is sorter?
    let shortend = new_string.len() < replaced_range_len;

    // This is how much longer or shorter the new range is.
    let len_difference = new_string.len().abs_diff(replaced_range_len);

    // No adjustment of annotations needed in case the replacement did not result in a change in length.
    if len_difference == 0 {
      return;
    }

    self.annotation.iter_mut().for_each(|annotation| {
      annotation.start = if annotation.start >= end {
        // For annotations starting after the replaced range, we move the start index by the difference in length.
        if shortend {
          // Move annotation to left
          annotation.start.saturating_sub(len_difference)
        } else {
          // Move annotation to right
          annotation.start.saturating_add(len_difference)
        }
        // If annotation start in between the insertion
      } else if annotation.start > start {
        // For annotations starting within the replaced range,
        // We move the start index by the difference in length, constrained to the beginning or end of the replaced range.
        if shortend {
          max(start, annotation.start.saturating_sub(len_difference))
        } else {
          min(end, annotation.start.saturating_add(len_difference))
        }
      } else {
        annotation.start
      };

      annotation.end = if annotation.end >= end {
        // For annotations ending after the replaced range, we move the end index by the difference in length.
        if shortend {
          annotation.end.saturating_sub(len_difference)
        } else {
          annotation.end.saturating_add(len_difference)
        }
      } else if annotation.end >= start {
        // For annotations ending within the replaced range, we move the end index by the difference in length, constrained to the beginning or end of the replaced range.
        if shortend {
          max(start, annotation.end.saturating_sub(len_difference))
        } else {
          min(end, annotation.end.saturating_add(len_difference))
        }
      } else {
        annotation.end
      };
    });

    self.annotation.retain(|annotation| {
      annotation.start < annotation.end && annotation.start < self.string.len()
    });
  }

  pub fn truncate_left_until(&mut self, until: ByteIdx) {
    self.replace(0, until, "");
  }

  pub fn truncate_right_from(&mut self, from: ByteIdx) {
    self.replace(from, self.string.len(), "");
  }
}

impl Display for AnnotatedString {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}", self.string)
  }
}

impl<'a> IntoIterator for &'a AnnotatedString {
  type Item = AnnotatedStringPart<'a>;
  type IntoIter = AnnotatedStringIterator<'a>;

  fn into_iter(self) -> Self::IntoIter {
    AnnotatedStringIterator {
      annotated_string: self,
      current_idx: 0,
    }
  }
}
