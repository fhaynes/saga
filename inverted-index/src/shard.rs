use std::fmt;

/// ShardType indicates if a particular Shard is a Primary or a Replica
pub enum ShardType {
    /// A Shard that is writable
    Primary,
    /// A Shard that is only readable and mirrors a specific Primary Shard
    Replica,
}

/// Struct that holds information about a Shard
pub struct Shard {
    index: String,
    shard_type: ShardType,
}

impl Into<String> for ShardType {
    /// Implements Into<String> for ShardType so we can easily convert it
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use inverted_index::shard::*;
    /// let shard_type = ShardType::Primary;
    /// let s: String = shard_type.into();
    /// ```
    fn into(self) -> String {
        match self {
            ShardType::Primary => String::from("primary"),
            ShardType::Replica => String::from("replica"),
        }
    }
}

impl fmt::Display for ShardType {
    /// Implements Display for ShardType
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ShardType::Primary => {
                write!(f, "primary")
            },
            &ShardType::Replica => {
                write!(f, "replica")
            }
        }
    }
}

impl Shard {
    /// Creates and returns a new Shard
    /// 
    /// # Arguments
    /// 
    /// * `index` - Name of the Index to which this Shard belongs
    /// * `shard_type` - Type of Shard this one is
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use inverted_index::shard::*;
    /// let new_shard = Shard::new("test_idx", ShardType::Primary);
    /// ```
    pub fn new<S: Into<String>>(index: S, shard_type: ShardType) -> Shard {
        Shard {
            index: index.into(),
            shard_type: shard_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shard() {
        let s = Shard::new("TestShard", ShardType::Primary);
        assert_eq!(s.index, "TestShard");
    }
}
