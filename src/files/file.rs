//! The file module groups everything related to a single file

use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use ansi_term::Style;

use crate::cli::Cli;
use crate::files::links::cache::Cache;
use crate::files::links::results::Results;

use super::links::Links;

/// Represents a file with possible links
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct File {
    /// Relative path to the file
    path: PathBuf,
}

impl File {
    /// Creates a new file form its path
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    /// Check all the links in this file (if there are).
    ///
    /// Then print and return the results.
    pub fn check(&self, cli: &Cli, cache: &mut Cache) -> Results {
        let results = Links::check(&self.path, cli, cache);

        println!("{}{}", self, results);

        results
    }

    /// Prints the links found in this file
    pub fn print_links(&self, cli: &Cli) {
        for link in Links::find(&self.path).unwrap_or_default() {
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
