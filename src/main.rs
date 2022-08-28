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

use std::{path::PathBuf, boxed::Box, error::Error};

use clap::Parser;
use cursive::{Cursive, CursiveExt, event::Key};

mod view;
mod cursor;
mod buffer;

use crate::{buffer::Buffer, view::EditorView};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(name = "FILE", help = "A path to a file to open.")]
    file_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let buffer = match args.file_path {
        Some(path) => Buffer::from_file(path)?,
        None => Buffer::new(),
    };

    let mut siv = Cursive::default();

    siv.add_fullscreen_layer(EditorView::new(buffer));
    siv.add_global_callback(Key::End, |c| c.quit());

    siv.run();

    Ok(())
}
