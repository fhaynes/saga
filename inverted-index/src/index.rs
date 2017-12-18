use std::fmt;
use std::error::Error;

use rusqlite;

///
pub struct InvertedIndex {
    /// The name of the Index. This could be something like "logs".
    name: String,
    /// An optional field to record the username or e-mail of who made the Index
    owner: Option<String>,
    /// The number of primary shards this InvertedIndex should have
    primary_shards: i32,
    /// The number of replicas this InvertedIndex should have
    replica_shards: i32,
}

impl InvertedIndex {
    /// Creates and returns a new InvertedIndex
    /// 
    /// # Arguments
    /// 
    /// * `name` - Name of the InvertedIndex
    /// * `owner` - Optional string that indicates who made the InvertedIndex
    /// * `primary` - Integer that indicates how many primary shards this InvertedIndex should have
    /// * `replica` - Integer that indicates how many replica shards this InvertedIndex should have
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use inverted_index::index::*;
    /// let new_index = InvertedIndex::new("test_idx", "fletcher", 1, 1);
    /// ```
    pub fn new<S: Into<String>, N: Into<i32>>(name: S, owner: S, primary: N, replica: N) -> InvertedIndex {
        InvertedIndex {
            name: name.into(),
            owner: Some(owner.into()),
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
    /// 
    /// # Arguments
    /// 
    /// * `msg` - Text of the error message
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use inverted_index::index::*;
    /// let new_index_error = InvertedIndexError::new("Test error");
    /// ```
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
