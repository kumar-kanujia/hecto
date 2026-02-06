#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum AnnotationType {
  /// A regular match
  Match,
  /// A match that is currently selected
  SelectedMatch,
  Number,
}
