//! Groups data structures related to a single link.
//!
//! This module contains a link wrapper and a link status.

use std::fmt::{Display, Formatter};

use ansi_term::Color::{Blue, Green, Red, Yellow};
use ansi_term::Style;
use regex::Match;
use reqwest::blocking::Client;

/// Represents the status of a link in the process
#[derive(Clone, PartialEq)]
pub enum LinkStatus {
    /// The link is alive (returned 2xx)
    Alive,
    /// The link is dead (returned other than 2xx).
    /// The value is the reason.
    ///
    /// This has a chance of being false because some websites block bots.
    Dead(String),
    /// There was a problem getting a response
    /// The value is the reason.
    Warn(String),
    /// Link was already tested and is in cache
    Cached,
    /// Link has been ignored
    Ignored,
}

/// Represents a single link that can be checked or formatted
#[derive(Clone, PartialEq)]
pub struct Link {
    #[doc(hidden)]
    link: String,
}

impl Link {
    /// Creates a new link from `s`.
    ///
    /// Link validity is not guaranteed.
    pub fn new(s: &String) -> Self {
        Self {
            link: s.to_string(),
        }
    }

    /// Creates a new link from a regex [`Match`].
    ///
    /// Link validity is not guaranteed.
    pub fn from_match(m: Match) -> Self {
        Self {
            link: m.as_str().to_string(),
        }
    }

    /// Check if link is alive
    ///
    /// Returns the status of the link.
    /// The link can be [alive](LinkStatus::Alive), [dead](LinkStatus::Dead) or generate a [warning](LinkStatus::Warn).
    pub fn alive(&self) -> LinkStatus {
        // TODO: try to make a common client
        let response = match Client::new().get(self.link.as_str()).send() {
            Ok(r) => r,
            Err(_) => {
                return LinkStatus::Warn("Too many redirections".to_string());
            }
        };

        if response.status().is_success() {
            LinkStatus::Alive
        } else {
            LinkStatus::Dead(response.status().to_string())
        }
    }

    /// Formats the link for an OK response
    pub fn ok(&self, reason: &str) -> String {
        format!("\t{} {} {}", Green.paint("[OK]"), self, reason)
    }

    /// Formats the link as a warning response
    pub fn warn(&self, reason: &str) -> String {
        format!(
            "\t{} {} {}",
            Yellow.paint("[WARN]"),
            self,
            Yellow.paint(reason)
        )
    }

    /// Formats the link as an error response
    pub fn err(&self, reason: &str) -> String {
        format!("\t{} {} {}", Red.paint("[ERR]"), self, Red.paint(reason))
    }

    /// Formats the link as cached
    pub fn cache(&self, reason: &str) -> String {
        format!(
            "\t{} {} {}",
            Green.dimmed().paint("[CACHE]"),
            Style::new().dimmed().paint(format!("{}", self)),
            Style::new().dimmed().paint(reason)
        )
    }

    /// Formats the link as ignored (i.e. by cli args)
    pub fn ignored(&self, reason: &str) -> String {
        format!(
            "\t{} {} {}",
            Style::new().dimmed().paint("[IGNORED]"),
            Style::new().dimmed().paint(format!("{}", self)),
            Style::new().dimmed().paint(reason)
        )
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Blue.underline().paint(&self.link))
    }
}
