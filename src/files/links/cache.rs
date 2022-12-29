use crate::files::links::link::Link;
use std::collections::HashSet;

pub struct Cache {
    data: HashSet<String>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    pub fn contains(&self, link: &Link) -> bool {
        self.data.contains(&link.to_string())
    }

    pub fn inserts(&mut self, link: &Link) {
        self.data.insert(link.to_string());
    }
}
