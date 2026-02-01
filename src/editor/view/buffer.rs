use crate::editor::{
  line::Line,
  view::{Location, fileinfo::FileInfo},
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
    if at.line_idx > self.height() {
      return;
    }

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

  /// Search of string slice in the buffer and return is's location
  pub fn search(&self, query: &str, from: Location) -> Option<Location> {
    // Search from within the current line until the bottom of the document
    for (line_idx, line) in self.lines.iter().enumerate().skip(from.line_idx) {
      let from_grapheme_idx = if line_idx == from.line_idx {
        //Ensure that we start in the current line from the desired position.
        from.grapheme_idx
      } else {
        //For every other line, we start at the begining of the line.
        0
      };

      if let Some(grapheme_idx) = line.search(query, from_grapheme_idx) {
        return Some(Location {
          grapheme_idx,
          line_idx,
        });
      }
    }

    // If noting is fround till the end of documnet
    // Search from the top of the document until (and including) the current line
    for (line_idx, line) in self.lines.iter().enumerate().take(from.line_idx) {
      if let Some(grapheme_idx) = line.search(query, 0) {
        // After wrapping around to the top, we can always start at the beginning of the line.
        return Some(Location {
          grapheme_idx,
          line_idx,
        });
      }
    }

    // If nothing is found, return None
    None
  }
}
