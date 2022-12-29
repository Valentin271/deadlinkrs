extern crate core;

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
