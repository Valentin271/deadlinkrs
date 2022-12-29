use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use ansi_term::Style;

use crate::cli::Cli;
use crate::files::links::cache::Cache;
use crate::files::links::results::Results;

use super::links::Links;

/// Respresents a file with possible links
pub struct File {
    path: PathBuf,
}

impl File {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn check(&self, cli: &Cli, cache: &mut Cache) -> Results {
        let results = Links::check(&self.path, cli, cache);

        println!("{}{}", self, results);

        results
    }

    /// Prints the links found in this file
    pub fn print_links(&self, cli: &Cli) {
        for link in Links::find(&self.path) {
            if !cli.ignore.contains(&link) {
                println!("\t{}", link);
            }
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Style::new()
                .dimmed()
                .paint(&self.path.display().to_string())
        )
    }
}
