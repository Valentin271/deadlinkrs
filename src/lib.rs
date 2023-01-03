#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Entry point module, contains the root application

use std::process::ExitCode;

use ansi_term::Color::{Green, Red};

pub use crate::cli::Cli;
pub use crate::files::file::File;
use crate::files::links::link::LinkStatus;
pub use crate::files::Files;

mod cli;
mod files;

/// Represents the application
pub struct App;

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Creates a new app
    pub fn new() -> Self {
        Self {}
    }

    /// Launches the app according to cli arguments
    pub fn run(&self, cli: &Cli) -> ExitCode {
        if cli.list || cli.dry {
            self.list(cli);

            return ExitCode::SUCCESS;
        }

        match self.check(cli) {
            Ok(_) => {
                println!("\n{}", Green.paint("No dead links !"));
                ExitCode::SUCCESS
            }
            Err(dead_links) => {
                println!(
                    "\n{}",
                    Red.paint(format!("Found {} dead links", dead_links))
                );
                ExitCode::FAILURE
            }
        }
    }

    /// Checks for dead links. Fails if at least one link is considered dead.
    pub fn check(&self, cli: &Cli) -> Result<(), usize> {
        let results = Files::new().check(cli);

        let dead_links = results.count_with(LinkStatus::Dead(String::new()));

        if dead_links == 0 {
            Ok(())
        } else {
            Err(dead_links)
        }
    }

    /// List files and links if asked
    pub fn list(&self, cli: &Cli) {
        for file in Files::find(cli) {
            println!("{}", file);
            if cli.dry {
                file.print_links(cli);
            }
        }
    }
}
