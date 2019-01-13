// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cursive::{
    Printer,
    vec::Vec2,
    view::View,
    theme::Effect,
    event::{Event, EventResult},
};

use crate::{cursor::Cursor, buffer::Buffer};

/// A text editor view.
pub struct EditorView {
    /// The buffer containing the text.
    buffer: Buffer,
    /// The position in the buffer as a character index.
    index: usize,
    /// The cursor position on the screen.
    cursor: Cursor,
    /// The window size.
    size: Vec2,
}

impl EditorView {
    /// Create a new `EditorView` instance from the specified buffer.
    pub fn new(buffer: Buffer) -> Self {
        Self {
            buffer,
            index: 0,
            cursor: Cursor::new(),
            size: Vec2::new(0, 0),
        }
    }

    /// Insert the character into the buffer after the current cursor location.
    pub fn insert(&mut self, chr: char) {
        self.buffer.insert_char(self.index, chr);
        // Update our cursor position.
        self.cursor.right();
    }

    /// Remove the specified amount of characters after the current cursor
    /// location.
    pub fn remove(&mut self, count: usize) {
        if self.index != 0 {
            self.buffer.remove(self.index - count, self.index);
        }
        // Update our cursor position.
        self.cursor.left();
    }
}

impl View for EditorView {
    fn draw(&self, printer: &Printer) {
        // TODO: Fill in empty space with a filler character.
        for (line_number, line) in self.buffer.lines().enumerate() {
            // Don't render more than we need to.
            if line_number > self.size.y {
                break;
            }
            printer.print((0, line_number), &line.to_string());
        }

        printer.with_effect(Effect::Reverse, |printer| {
            // Print the cursor to the screen.
            printer.print(
                &self.cursor,
                &self.cursor.char(&self.buffer).to_string()
            );

            // TODO: Make this a proper modeline.
            printer.print(
                (0, 50),
                &format!(
                    "Pos: {:?}, cursor: {:?}",
                    self.index,
                    (self.cursor.x(), self.cursor.y())
                ),
            );
            printer.print((0, 51), &format!("Size: {:?}", self.size));
        });
    }

    fn layout(&mut self, size: Vec2) {
        self.size = size;
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        // Set the required size to the largest available, since we want the
        // editor to take up the entire view.
        constraint
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        use cursive::event::{Event::*, Key::*};

        match event {
            // Editing
            Char(key) => self.insert(key),
            Key(Enter) => self.insert('\n'),
            Key(Backspace) => self.remove(1),

            // Movement
            Key(Left) => self.cursor.left(),
            Key(Right) if !self.cursor.is_past_end(&self.buffer) => self.cursor.right(),
            Key(Up) => self.cursor.up(),
            Key(Down) => self.cursor.down(),

            _ => return EventResult::Ignored,
        }

        // Update our position in the buffer.
        self.index = self.cursor.index(&self.buffer);

        EventResult::Consumed(None)
    }
}
