struct Key {
    id: u32,
    metadata: Option<String>,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Citation {
    author: String,
    year: u32,
}

impl PartialOrd for Citation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Code pour comparer
    }
}
