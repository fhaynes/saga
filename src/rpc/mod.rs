pub mod messages;
pub mod node;
pub mod db;

/// Cluster represents a collection of Nodes
pub struct Cluster {
    name: String
}


impl Cluster {
    /// Creates and returns a new Cluster
    pub fn new<S: Into<String>>(name: S) -> Cluster {
        Cluster{
            name: name.into()
        }
    }
}

