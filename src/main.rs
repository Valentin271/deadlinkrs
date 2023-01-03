//! Search for dead links in any kind of file

use std::process::ExitCode;

use human_panic::setup_panic;

use deadlinkrs::{App, Cli};

fn main() -> ExitCode {
    setup_panic!();

    let app = App::new();

    app.run(&Cli::build())
}
