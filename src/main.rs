// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{boxed::Box, error::Error};

use clap::{
    App,
    AppSettings,
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

use crate::{view::EditorView, buffer::Buffer};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("FILE").help("The file to edit").index(1))
        .get_matches();

    let mut siv = Cursive::default();

    // It is safe to use unwrap here since the FILE parameter is required.
    let buffer = Buffer::from_file(&matches.value_of("FILE").unwrap())?;
    siv.add_fullscreen_layer(EditorView::new(buffer));
    siv.add_global_callback(Key::End, |c| c.quit());

    siv.run();

    Ok(())
}
