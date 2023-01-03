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

impl Default for Results {
    fn default() -> Self {
        Self::new()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let results = Results::new();

        assert_eq!(results.keys.len(), 0);
        assert_eq!(results.values.len(), 0);
    }

    #[test]
    fn inserts() {
        let mut results = Results::new();
        let link = Link::new(&"https://example.com".to_string());

        results.inserts(&link, Alive);

        assert_eq!(results.keys.len(), 1);
        assert_eq!(results.values.len(), 1);

        assert_eq!(results.keys.first().unwrap(), &link);
        assert_eq!(results.values.first().unwrap(), &Alive);
    }

    #[test]
    fn merge() {
        // Creating results
        let mut results1 = Results::new();
        let mut results2 = Results::new();

        let link = Link::new(&"https://example.com".to_string());
        let link2 = Link::new(&"https://example.com/2".to_string());

        results1.inserts(&link, Alive);
        results2.inserts(&link2, Ignored);

        // Merge
        results1.merge(results2);

        // Check len
        assert_eq!(results1.keys.len(), 2);
        assert_eq!(results1.values.len(), 2);

        // Check added values
        assert!(results1.keys.contains(&link2));
        assert!(results1.values.contains(&Ignored));
    }

    #[test]
    fn count_with() {
        let mut results = Results::new();
        let link = Link::new(&"https://example.com".to_string());

        results.inserts(&link, Alive);

        assert_eq!(results.count_with(Alive), 1);
        assert_eq!(results.count_with(Ignored), 0);
    }

    #[test]
    /// Enum value should not matter
    fn count_with_enum_value() {
        let mut results = Results::new();
        let link = Link::new(&"https://example.com".to_string());

        results.inserts(&link, Warn("Too many redirection".to_string()));

        assert_eq!(results.count_with(Warn("".to_string())), 1);
    }
}
