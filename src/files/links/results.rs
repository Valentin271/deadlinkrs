//! The results module groups data related to check results.

use std::fmt::{Display, Formatter};
use std::mem::discriminant;

use crate::files::links::link::LinkStatus::{Alive, Cached, Dead, Ignored, Warn};
use crate::files::links::link::{Link, LinkStatus};

/// Represents the results links check
pub struct Results {
    /// Links checked
    keys: Vec<Link>,
    /// Status returned for each link.
    ///
    /// Correspondence with `keys` is kept by index.
    values: Vec<LinkStatus>,
}

impl Results {
    /// Creates a new empty results set.
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Merges two sets of results together.
    ///
    /// `other` set is inserted **after** `self`.
    pub fn merge(&mut self, mut other: Self) {
        self.keys.append(&mut other.keys);
        self.values.append(&mut other.values);
    }

    /// Inserts a new result
    pub fn inserts(&mut self, link: &Link, status: LinkStatus) {
        self.keys.push(link.clone());
        self.values.push(status);
    }

    /// Count the number of results with the given status.
    ///
    /// Only enum discriminant is taken into account.
    pub fn count_with(&self, status: LinkStatus) -> usize {
        self.values
            .clone()
            .into_iter()
            .filter(|s| discriminant(s) == discriminant(&status))
            .count()
    }
}

impl Display for Results {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();

        for (i, key) in self.keys.iter().enumerate() {
            s = format!(
                "{}\n{}",
                s,
                match self.values.get(i).unwrap() {
                    Alive => key.ok(""),
                    Dead(r) => key.err(r),
                    Warn(r) => key.warn(r),
                    Cached => key.cache(""),
                    Ignored => key.ignored(""),
                }
            );
        }

        write!(f, "{}", s)
    }
}
