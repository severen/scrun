// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{boxed::Box, error::Error};

use cursive::Cursive;

mod view;
mod cursor;
mod buffer;

use crate::{view::EditorView, buffer::Buffer};

fn main() -> Result<(), Box<dyn Error>> {
    let mut siv = Cursive::default();

    let buffer = Buffer::from_file("test/large.txt")?;
    siv.add_fullscreen_layer(EditorView::new(buffer));

    siv.run();

    Ok(())
}
