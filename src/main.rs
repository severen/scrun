// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![deny(bare_trait_objects, elided_lifetimes_in_paths)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_qualifications,
)]

use std::{boxed::Box, error::Error};

use clap::{
    App,
    Arg,
    crate_name,
    crate_description,
    crate_version,
    crate_authors,
};
use cursive::{Cursive, event::Key};

mod view;
mod cursor;
mod buffer;

use crate::{buffer::Buffer, view::EditorView};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("FILE").help("A file to open for editing").index(1))
        .get_matches();

    let buffer = match matches.value_of("FILE") {
        Some(file) => Buffer::from_file(file)?,
        None => Buffer::new(),
    };

    let mut siv = Cursive::default();

    siv.add_fullscreen_layer(EditorView::new(buffer));
    siv.add_global_callback(Key::End, |c| c.quit());

    siv.run();

    Ok(())
}
