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
