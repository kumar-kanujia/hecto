use crate::{editor::annotatedstring::annotationtype::AnnotationType, prelude::*};

// clippy::struct_field_names: naming the field `type` is disallowed due to type being a keyword.
/// Represents annotation on the annotated string
#[derive(Debug, Copy, Clone)]
#[allow(clippy::struct_field_names)]
pub struct Annotation {
  pub annotation_type: AnnotationType,
  pub start: ByteIdx,
  pub end: ByteIdx,
}
