// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cursive::Vec2;
use derive_more::Display;

use crate::buffer::Buffer;

/// The cursor on the screen.
#[derive(Display, Debug)]
#[display(fmt = "({}, {})", x, y)]
pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    /// Create a new `Cursor` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `Cursor` instance at the specified position.
    pub fn at(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Get the current cursor position on the x-axis, or in other words, the
    /// cursor's current column.
    pub fn x(&self) -> usize {
        self.x
    }

    /// Get the current cursor position on the y-axis, or in other words, the
    /// cursor's current row.
    pub fn y(&self) -> usize {
        self.y
    }

    /// Move the cursor up by one row.
    pub fn up(&mut self) {
        // Going above the first line makes no sense and probably would cause a
        // panic.
        if self.y != 0 {
            self.y -= 1;
        }
    }

    /// Move the cursor down by one row.
    pub fn down(&mut self) {
        // TODO: Prevent movement past the last line of the buffer.
        self.y += 1;
    }

    /// Move the cursor left by one column.
    pub fn left(&mut self) {
        // We follow Vim's behaviour of preventing movement past the start of
        // the line instead of wrapping around to the end of the line above.
        if self.x != 0 {
            self.x -= 1;
        }
    }

    /// Move the cursor right by one column.
    pub fn right(&mut self) {
        self.x += 1;
    }

    /// Get the current index in the provided buffer at the cursor position.
    pub fn index(&self, buffer: &Buffer) -> usize {
        buffer.line_to_char(self.y) + self.x
    }

    // TODO: Make this a function on buffer instead.
    /// Get the character in the provided buffer at the cursor position.
    pub fn char(&self, buffer: &Buffer) -> char {
        // TODO: Figure out how to remove this extra index call.
        //       When this code was previously part of `EditorView`, we could just
        //       access the stored index directly.
        buffer.get_char(self.index(buffer))
    }

    /// Check if the cursor is past the end of the current line in the provided
    /// buffer.
    pub fn is_past_end(&self, buffer: &Buffer) -> bool {
        let current_line = buffer.get_line(self.y);
        // We check against the length of the line minus 1 in order to not
        // count the newline character.
        self.x == current_line.len_chars() - 1
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl From<&Cursor> for Vec2 {
    fn from(cursor: &Cursor) -> Self {
        Self::new(cursor.x, cursor.y)
    }
}
