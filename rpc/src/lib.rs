extern crate rusqlite;
extern crate uuid;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod messages;
pub mod node;
pub mod db;

use std::sync::{Arc,Mutex,mpsc};

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

/// Struct that holds Channels to various entities. 
pub struct Switchboard {
    pub node_tx: Arc<Mutex<mpsc::Sender<messages::Message>>>,
}

impl Switchboard {
    pub fn new(node_tx: Arc<Mutex<mpsc::Sender<messages::Message>>>) -> Switchboard {
        Switchboard{
            node_tx: node_tx,
        }
    }
}