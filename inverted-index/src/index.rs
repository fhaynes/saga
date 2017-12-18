use std::fmt;
use std::error::Error;

use rusqlite;


pub struct InvertedIndex {
    name: String,
    owner: String,
    primary_shards: i32,
    replica_shards: i32,
}

impl InvertedIndex {
    fn new<S: Into<String>, N: Into<i32>>(name: S, owner: S, primary: N, replica: N) -> InvertedIndex {
        InvertedIndex {
            name: name.into(),
            owner: owner.into(),
            primary_shards: primary.into(),
            replica_shards: replica.into(),
        }
    }
}

impl fmt::Display for InvertedIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for InvertedIndexError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct InvertedIndexError {
    details: String,
}

impl InvertedIndexError {
    /// Creates and returns a new InvertedIndexError
    pub fn new(msg: &str) -> InvertedIndexError {
        InvertedIndexError { details: msg.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_inverted_index() {
        let index = InvertedIndex::new("Test", "TestOwner", 1, 1);
        assert_eq!(index.name, "Test");
    }
}
