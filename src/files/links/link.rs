use std::fmt::{Display, Formatter};

use ansi_term::Color::{Blue, Green, Red, Yellow};
use ansi_term::Style;
use regex::Match;
use reqwest::blocking::Client;

#[derive(Clone, PartialEq)]
pub enum LinkStatus {
    Alive,
    Dead(String),
    Warn(String),
    Cached,
    Ignored,
}

/// Represents a single link
#[derive(Clone, PartialEq)]
pub struct Link {
    link: String,
}

impl Link {
    pub fn new(s: &String) -> Self {
        Self {
            link: s.to_string(),
        }
    }

    pub fn from_match(m: Match) -> Self {
        Self {
            link: m.as_str().to_string(),
        }
    }

    /// Check if link is alive
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

    pub fn ok(&self, reason: &str) -> String {
        format!("\t{} {} {}", Green.paint("[OK]"), self, reason)
    }

    pub fn warn(&self, reason: &str) -> String {
        format!(
            "\t{} {} {}",
            Yellow.paint("[WARN]"),
            self,
            Yellow.paint(reason)
        )
    }

    pub fn err(&self, reason: &str) -> String {
        format!("\t{} {} {}", Red.paint("[ERR]"), self, Red.paint(reason))
    }

    pub fn cache(&self, reason: &str) -> String {
        format!(
            "\t{} {} {}",
            Green.dimmed().paint("[CACHE]"),
            Style::new().dimmed().paint(format!("{}", self)),
            Style::new().dimmed().paint(reason)
        )
    }

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
