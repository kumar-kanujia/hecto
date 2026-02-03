use crate::editor::annotatedstring::annotationtype::AnnotationType;

// clippy::struct_field_names: naming the field `type` is disallowed due to type being a keyword.
/// Represents annotation on the annotated string
#[derive(Debug, Copy, Clone)]
#[allow(clippy::struct_field_names)]
pub struct Annotation {
  pub annotation_type: AnnotationType,
  pub start_byte_idx: usize,
  pub end_byte_idx: usize,
}
