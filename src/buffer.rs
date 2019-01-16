// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    io,
    fs::{self, File},
    path::{Path, PathBuf},
};

use ropey::{
    Rope,
    RopeSlice,
    iter::{Chars, Lines},
};

/// A representation of a buffer of text, along with its associated metadata.
#[derive(Debug)]
pub struct Buffer {
    /// The rope that contains the text itself.
    text: Rope,
    /// The path to the file that the buffer represents.
    ///
    /// This is represented as an `Option` due to the fact that not every buffer
    /// neccessarily has a file associated with it (e.g. temporary buffers, new
    /// unsaved buffers).
    path: Option<PathBuf>,
    /// The edit state of the buffer.
    dirty: bool,
}

impl Buffer {
    /// Create a new `Buffer` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `Buffer` instance from the specified file.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let text = Rope::from_reader(
            &mut io::BufReader::new(File::open(&path)?)
        )?;

        Ok(Self {
            text,
            // In theory this should never raise an error, as its error
            // conditions are already covered when opening the file.
            path: Some(fs::canonicalize(path)?),
            ..Self::default()
        })
    }

    /// Get the path to the underlying file, if there is one.
    pub fn path(&self) -> Option<&Path> {
        match &self.path {
            Some(path) => Some(&path),
            None => None,
        }
    }

    /// Create an iterator over the characters in the buffer.
    pub fn chars(&self) -> Chars {
        self.text.chars()
    }

    /// Create an iterator over the lines in the buffer.
    pub fn lines(&self) -> Lines {
        self.text.lines()
    }

    /// Get the number of lines in the buffer.
    pub fn num_lines(&self) -> usize {
        self.text.len_lines()
    }

    /// Get the number of characters in the buffer.
    pub fn num_chars(&self) -> usize {
        self.text.len_chars()
    }

    /// Get the character at the given index.
    pub fn get_char(&self, index: usize) -> char {
        self.text.char(index)
    }

    /// Get the line at the specified line index.
    pub fn get_line(&self, line_index: usize) -> RopeSlice {
        self.text.line(line_index)
    }

    /// Get the character index of the first character at the given line index.
    pub fn line_to_char(&self, line_index: usize) -> usize {
        self.text.line_to_char(line_index)
    }

    /// Insert some text after the given index.
    pub fn insert(&mut self, index: usize, text: &str) {
        self.text.insert(index, text);
        self.dirty = true;
    }

    /// Insert a character after the given index.
    pub fn insert_char(&mut self, index: usize, chr: char) {
        self.text.insert_char(index, chr);
        self.dirty = true;
    }

    /// Delete text in the specified region.
    pub fn remove(&mut self, start: usize, end: usize) {
        self.text.remove(start..end);
        self.dirty = true;
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            text: Rope::new(),
            path: None,
            dirty: false,
        }
    }
}
