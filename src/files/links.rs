//! Module for anything link related

use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;

use regex::Regex;

use link::Link;
use link::LinkStatus;

use crate::cli::Cli;
use crate::files::links::cache::Cache;
use crate::files::links::results::Results;

pub mod cache;
pub mod link;
pub mod results;

/// A list of links that can be tested
pub struct Links;

impl Links {
    /// Finds the links in the file at `path`.
    ///
    /// Uses a regex to do so.
    pub fn find(path: &PathBuf) -> io::Result<Vec<Link>> {
        // TODO: define some sort of constant
        let regex: Regex = Regex::new(
            "https?://(?:[[:alnum:]]+\\.)?[[:alnum:]]+\\.[[:alpha:]]{2,3}/?(?:[[:alnum:]]|[-$_.+!*/&?%=@,:])*",
        )
            .expect("Valid regex");

        let content = read_to_string(path)?;

        Ok(regex.find_iter(&content).map(Link::from_match).collect())
    }

    /// Check every link in `path` found by [`Links::find`].
    ///
    /// Links in cache are skipped.  
    /// Links ignored by cli arguments are skipped.  
    /// Links determined alive are added to cache.
    ///
    /// Returns the results of this file's check.
    pub fn check(path: &PathBuf, cli: &Cli, cache: &mut Cache) -> Results {
        let mut results = Results::new();

        for link in Links::find(path).unwrap_or_default() {
            if cli.ignore.contains(&link) {
                results.inserts(&link, LinkStatus::Ignored);
                continue;
            }

            if cache.contains(&link) {
                results.inserts(&link, LinkStatus::Cached);
                continue;
            }

            let status = link.alive();

            if status == LinkStatus::Alive {
                cache.inserts(&link);
            }

            results.inserts(&link, status);
        }

        results
    }
}
