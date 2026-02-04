use crate::editor::annotatedstring::annotationtype::AnnotationType;

/// A part of `AnnotatedString`
#[derive(Debug)]
pub struct AnnotatedStringPart<'a> {
  pub string: &'a str,
  pub annotation_type: Option<AnnotationType>,
}
