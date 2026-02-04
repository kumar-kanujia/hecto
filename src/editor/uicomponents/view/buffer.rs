use crate::editor::{
  line::Line,
  uicomponents::view::{fileinfo::FileInfo, location::Location},
};

use std::{
  fs::{File, read_to_string},
  io::{Error, Write},
};

#[derive(Default)]
pub struct Buffer {
  /// Store line as an vector
  pub lines: Vec<Line>,
  pub file_info: FileInfo,
  /// Marked true if there is chnage in original buffer
  pub dirty: bool,
}

impl Buffer {
  pub fn load(file_name: &str) -> Result<Self, Error> {
    let contents = read_to_string(file_name)?;
    let mut lines = Vec::new();
    for value in contents.lines() {
      lines.push(Line::from(value));
    }
    Ok(Self {
      lines,
      file_info: FileInfo::from(file_name),
      dirty: false,
    })
  }

  /// Save the buffer in the give file
  fn save_to_file(&self, file_info: &FileInfo) -> Result<(), Error> {
    if let Some(file_path) = &file_info.get_path() {
      let mut file = File::create(file_path)?;
      for line in &self.lines {
        writeln!(file, "{line}")?;
      }
    }
    Ok(())
  }

  /// Save the buffer in the file by creating new file with ` file_name `
  pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
    let file_info = FileInfo::from(file_name);
    self.save_to_file(&file_info)?;
    self.file_info = file_info;

    self.dirty = false;
    Ok(())
  }

  /// Save the existing file
  pub fn save(&mut self) -> Result<(), Error> {
    self.save_to_file(&self.file_info)?;
    self.dirty = false;
    Ok(())
  }

  /// Return True if buffer is empty
  pub fn is_empty(&self) -> bool {
    self.lines.is_empty()
  }

  pub const fn is_file_loaded(&self) -> bool {
    self.file_info.has_path()
  }

  /// Return total height covered by buffer
  pub fn height(&self) -> usize {
    self.lines.len()
  }

  /// Insert a character in a line on at at
  pub fn insert_char(&mut self, character: char, at: Location) {
    // We don't insert anything more than line below the document
    debug_assert!(at.line_idx <= self.height());

    // At the end of document then add a new line
    if at.line_idx == self.height() {
      self.lines.push(Line::from(&character.to_string()));
      self.dirty = true;
    } else if let Some(line) = self.lines.get_mut(at.line_idx) {
      // If we are at the end of line, just insert the character
      line.insert_char(character, at.grapheme_idx);
      self.dirty = true;
    }
  }

  /// Delete a char given at location
  pub fn delete(&mut self, at: Location) {
    // Check if we are at a valid line
    if let Some(line) = self.lines.get(at.line_idx) {
      // Check if we are at the end of current line and there's atleast next line available
      if at.grapheme_idx >= line.grapheme_count() && self.height() > at.line_idx.saturating_add(1) {
        let next_line = self.lines.remove(at.line_idx.saturating_add(1));

        // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
        #[allow(clippy::indexing_slicing)]
        self.lines[at.line_idx].append(&next_line);
        self.dirty = true;
      } else if at.grapheme_idx < line.grapheme_count() {
        // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
        #[allow(clippy::indexing_slicing)]
        self.lines[at.line_idx].delete(at.grapheme_idx);
        self.dirty = true;
      }
    }
  }

  /// Insert a new given at location
  pub fn insert_newline(&mut self, at: Location) {
    // If we are at the end of document, insert an empty line.
    if at.line_idx == self.height() {
      self.lines.push(Line::default());
      self.dirty = true;
    }
    // If we are in middle of document
    else if let Some(line) = self.lines.get_mut(at.line_idx) {
      // Split the current line
      let new = line.split(at.grapheme_idx);
      // Add the splitted part as next line
      self.lines.insert(at.line_idx.saturating_add(1), new);
      self.dirty = true;
    }
  }

  pub fn search_forward(&self, query: &str, from: Location) -> Option<Location> {
    if query.is_empty() {
      return None;
    }

    let mut is_first = true;

    for (line_idx, line) in self
      .lines
      .iter()
      .enumerate()
      .cycle()
      .skip(from.line_idx)
      .take(self.lines.len().saturating_add(1))
    //taking one more, to search the current line twice (once from the middle, once from the start)
    {
      let from_grapheme_idx = if is_first {
        is_first = false;
        from.grapheme_idx
      } else {
        0
      };

      if let Some(grapheme_idx) = line.search_forward(query, from_grapheme_idx) {
        return Some(Location {
          grapheme_idx,
          line_idx,
        });
      }
    }
    None
  }

  pub fn search_backward(&self, query: &str, from: Location) -> Option<Location> {
    if query.is_empty() {
      return None;
    }
    let mut is_first = true;
    for (line_idx, line) in self
      .lines
      .iter()
      .enumerate()
      .rev()
      .cycle()
      .skip(
        self
          .lines
          .len()
          .saturating_sub(from.line_idx)
          .saturating_sub(1),
      )
      .take(self.lines.len().saturating_add(1))
    {
      let from_grapheme_idx = if is_first {
        is_first = false;
        from.grapheme_idx
      } else {
        line.grapheme_count()
      };
      if let Some(grapheme_idx) = line.search_backward(query, from_grapheme_idx) {
        return Some(Location {
          grapheme_idx,
          line_idx,
        });
      }
    }
    None
  }
}
