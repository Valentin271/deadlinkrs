use crate::files::links::link::LinkStatus::{Alive, Cached, Dead, Ignored, Warn};
use crate::files::links::link::{Link, LinkStatus};
use std::fmt::{Display, Formatter};
use std::mem::discriminant;

/// Represents the results of a file links check
pub struct Results {
    keys: Vec<Link>,
    values: Vec<LinkStatus>,
}

impl Results {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Merges two sets of results together
    pub fn merge(&mut self, mut other: Self) {
        self.keys.append(&mut other.keys);
        self.values.append(&mut other.values);
    }

    pub fn inserts(&mut self, link: &Link, status: LinkStatus) {
        self.keys.push(link.clone());
        self.values.push(status);
    }

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
