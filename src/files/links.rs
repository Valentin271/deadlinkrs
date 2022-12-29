use std::fs::read_to_string;
use std::path::PathBuf;

use regex::Regex;

use crate::cli::Cli;
use link::Link;
use link::LinkStatus;

use crate::files::links::cache::Cache;
use crate::files::links::results::Results;

pub mod cache;
pub mod link;
pub mod results;

/// A list of links that can be tested
pub struct Links;

impl Links {
    pub fn find(path: &PathBuf) -> Vec<Link> {
        // TODO: define some sort of constant
        let regex: Regex = Regex::new(
            "https?://(?:[[:alnum:]]+\\.)?[[:alnum:]]+\\.[[:alpha:]]{2,3}/?(?:[[:alnum:]]|[-$_.+!*/&?%=@,:])*",
        )
            .expect("Valid regex");

        match read_to_string(path) {
            Ok(content) => regex.find_iter(&content).map(Link::from_match).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub fn check(path: &PathBuf, cli: &Cli, cache: &mut Cache) -> Results {
        let mut results = Results::new();

        for link in Links::find(path) {
            if cli.ignore.contains(&link) {
                results.inserts(&link, LinkStatus::Ignored);
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
