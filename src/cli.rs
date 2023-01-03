//! This module contains cli arguments handling

use clap::{arg, command, ArgMatches};
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};

use crate::files::links::link::Link;

/// Represents the arguments given to the cli
pub struct Cli {
    /// Paths to recursively search
    pub path: Vec<String>,
    /// Set of globs to validate files
    pub glob: GlobSet,
    /// Set of globs to exclude files
    pub exclude: GlobSet,
    /// Links to ignore checking
    pub ignore: Vec<Link>,
    /// Hidden files and directories are processed
    pub hidden: bool,
    /// Only list files that would be processed
    pub list: bool,
    /// Only list links that would be checked
    pub dry: bool,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            path: vec![String::from(".")],
            glob: Self::globs(vec![String::from("**")].iter()),
            exclude: GlobSet::default(),
            ignore: Vec::new(),
            hidden: false,
            list: false,
            dry: false,
        }
    }
}

impl Cli {
    /// Creates a new cli arguments wrapper ready to use.
    pub fn build() -> Self {
        let matches: ArgMatches = command!()
            .arg(arg!([path]... "Path to look for files").default_value("."))
            .arg(arg!(-g --glob <glob>... "Unix-style glob to filter files").default_value("**"))
            .arg(arg!(-e --exclude <glob>... "Unix-style glob to exclude from selection"))
            .arg(arg!(--hidden "Includes hidden files and directories"))
            .arg(arg!(-i --ignore <url>... "URL to ignore"))
            .arg(arg!(--list "List searched files and exits"))
            .arg(arg!(--dry "Extract and print URLs that should be requested but don't send requests"))
            .get_matches();

        Self {
            path: matches
                .get_many::<String>("path")
                .expect("path arguments should be valid")
                .map(String::from)
                .collect(),
            glob: Cli::globs(
                matches
                    .get_many::<String>("glob")
                    .expect("glob arguments should be valid"),
            ),
            exclude: Cli::exclude_globs(matches.get_many::<String>("exclude").unwrap_or_default()),
            ignore: matches
                .get_many::<String>("ignore")
                .unwrap_or_default()
                .map(Link::new)
                .collect(),
            hidden: matches.get_flag("hidden"),
            list: matches.get_flag("list"),
            dry: matches.get_flag("dry"),
        }
    }

    /// Build the set of globs to test the files against
    fn globs<'a>(globs: impl Iterator<Item = &'a String>) -> GlobSet {
        let mut builder = GlobSetBuilder::new();

        for glob in globs {
            builder.add(
                GlobBuilder::new(glob.as_str())
                    .literal_separator(true)
                    .build()
                    .expect("Glob pattern should be correct"),
            );
        }

        builder.build().expect("Glob patterns should be correct")
    }

    /// Build the set of globs to exclude files
    fn exclude_globs<'a>(globs: impl Iterator<Item = &'a String>) -> GlobSet {
        let mut exclude_builder = GlobSetBuilder::new();

        for glob in globs {
            exclude_builder.add(
                GlobBuilder::new(glob.as_str())
                    .literal_separator(true)
                    .build()
                    .expect("Exclude glob pattern should be correct"),
            );
        }
        exclude_builder
            .build()
            .expect("Exclude glob patterns should be correct")
    }
}
