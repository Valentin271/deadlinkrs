#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Search for dead links in any kind of file

use std::process::ExitCode;

use human_panic::setup_panic;

use crate::app::App;

mod app;
mod cli;
mod files;

fn main() -> ExitCode {
    setup_panic!();

    let mut app = App::new();

    app.run()
}
