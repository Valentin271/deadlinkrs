//! Module defining cache related utilities.
//!
//! The cache aims to not check a link twice.
//! For now, only valid links are stored and invalid links are re-checked every time.

use std::collections::HashSet;

use crate::files::links::link::Link;

/// Represents the links cache
pub struct Cache {
    #[doc(hidden)]
    data: HashSet<String>,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    /// Creates a new empty cache
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    /// Check if the cache contains the given link
    pub fn contains(&self, link: &Link) -> bool {
        self.data.contains(&link.to_string())
    }

    /// Inserts a new link in the cache
    pub fn inserts(&mut self, link: &Link) {
        self.data.insert(link.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn link() -> Link {
        Link::new(&"https://example.com".to_string())
    }

    #[test]
    fn new() {
        let cache = Cache::new();

        assert_eq!(cache.data.len(), 0);
    }

    #[test]
    fn inserts() {
        let mut cache = Cache::new();

        cache.inserts(&link());

        assert_eq!(cache.data.len(), 1);
    }

    #[test]
    fn not_contain_when_new() {
        let cache = Cache::new();

        assert!(!cache.contains(&Link::new(&"".to_string())));
        assert!(!cache.contains(&link()));
    }

    #[test]
    fn contains_after_inserted() {
        let mut cache = Cache::new();

        cache.inserts(&link());

        assert!(cache.contains(&link()));
    }

    #[test]
    fn not_contains_different_link() {
        let mut cache = Cache::new();

        cache.inserts(&link());

        assert!(!cache.contains(&Link::new(&"".to_string())));
    }
}
