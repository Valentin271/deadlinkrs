use std::process::ExitCode;

use ansi_term::Color::{Green, Red};

use crate::cli::Cli;
use crate::files::links::link::LinkStatus;
use crate::files::Files;

pub struct App {
    cli: Cli,
}

impl App {
    /// Creates a new app
    pub fn new() -> Self {
        Self { cli: Cli::new() }
    }

    /// Launches the app
    pub fn run(&mut self) -> ExitCode {
        if self.cli.dry {
            self.dry()
        } else if self.cli.list {
            self.list()
        } else {
            self.check()
        }
    }

    /// Checks for dead links
    pub fn check(&mut self) -> ExitCode {
        let results = Files::new().check(&self.cli);

        let dead_links = results.count_with(LinkStatus::Dead(String::new()));

        if dead_links == 0 {
            println!("\n{}", Green.paint("No dead links !"));
            ExitCode::SUCCESS
        } else {
            println!(
                "\n{}",
                Red.paint(format!("Found {} dead links", dead_links))
            );
            ExitCode::FAILURE
        }
    }

    /// List files that would be searched
    pub fn list(&self) -> ExitCode {
        Files::list(&self.cli);
        ExitCode::SUCCESS
    }

    /// List links that would be checked
    pub fn dry(&self) -> ExitCode {
        Files::dry(&self.cli);
        ExitCode::SUCCESS
    }
}
