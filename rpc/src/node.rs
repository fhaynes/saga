
use std;
use std::io::{Read,Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, fs};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex, mpsc};

use rusqlite::Connection;
use serde_json;

use messages::{Message, MessageType};
use db::MetadataDB;

/// Node is an individual server within a Cluster
pub struct Node {
    pub config: NodeConfiguration,
    pub db: Connection,
    pub metadata_connection: Option<TcpStream>
}

/// Contains the configuration data for creating a new Node
/// This is used due to Node needed so many parameters
pub struct NodeConfiguration {
    /// Name of the node
    pub name: String,
    /// Address of the metadata server we are going to connect to
    pub metadata_address: String,
    /// Path on the local machine that should be used for data. Primarily used
    /// to place the metadata db
    pub data_path: String,
    /// Port of the metadata server we are going to connect to
    pub metadata_port: u16,
    /// Tracks if we are a metadata server or not
    pub am_metadata_server: bool,
    /// Channel for this Node to listen for Messages
    pub rx: Arc<Mutex<mpsc::Receiver<Message>>>,
    /// The address that this node will listen on for RPC connections
    pub rpc_address: String,
    /// The port that this node will listen on for RPC connections
    pub rpc_port: u16
}


impl Node {
    /// Returns a new Node
    /// 
    /// # Arguments
    /// * `config` - A NodeConfiguration struct
    ///
    /// # Example
    ///
    /// ```
    ///  use rpc::node::{Node, NodeConfiguration};
    ///  use rpc::messages::Message;
    ///  use std::sync::{Arc, mpsc, Mutex};
    ///  let (_my_node_tx, my_node_rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    ///  let new_config = NodeConfiguration {
    ///    metadata_address: String::from("localhost"),
    ///    data_path: String::from("memory"),
    ///    metadata_port: 5000,
    ///    am_metadata_server: true,
    ///    rx: Arc::new(Mutex::new(my_node_rx))
    ///  };
    /// let _new_node = Node::new(new_config);
    /// ```
    pub fn new(config: NodeConfiguration) -> Node {
        let result = fs::create_dir_all(&config.data_path);
        let metadata_db: Connection;
        if result.is_err() {
            println!("Unable to create the directory for the metadata database. It will be created in memory.");
            metadata_db = Connection::open_in_memory().unwrap();
        } else {
            metadata_db = match Connection::open(&config.data_path) {
                Ok(conn) => {
                    conn
                },
                Err(e) => {
                    println!("There was an error opening the on-disk database: {:?}. It will be created in memory.", e);
                   Connection::open_in_memory().unwrap()
                }
            };
        }

        let metadata_connection: Option<TcpStream>;

        // TODO: This should be factored out into a function
        if config.am_metadata_server {
            metadata_connection = None;

        } else {
            match TcpStream::connect(config.metadata_address.clone() + ":" + &config.metadata_port.to_string()) {
                Ok(conn) => {
                    println!("Connected to metadata server at: {}:{}", config.metadata_address, &config.metadata_port);
                    metadata_connection = Some(conn);
                },
                Err(e) => {
                    println!("There was an error connecting to the metadata server: {:?}", e);
                    metadata_connection = None;
                },
            };
        }

        Node {
            config: config,
            db: metadata_db,
            metadata_connection: metadata_connection,
        }
    }


    /// Starts the RPC server for a `Node`. This binds to a port and listens for messages from other
    /// Nodes on the network. When it gets one, it deserializes it and sends it to a handler via
    /// a Channel.
    ///
    /// # Arguments
    /// 
    /// * `bind_host` - The interface to which the server should bind
    /// * `bind_port` - The port to which the server should bind
    /// * `tx` - The Channel down which the server sends decoded `Messages`
    ///
    /// # Example
    ///
    /// ```
    ///  // use rpc::node::Node;
    ///  use rpc::messages::Message;
    ///  use std::sync::{Arc, Mutex, mpsc};
    ///  let (_tx, _rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    ///  // TODO: Figure out how to actually test this
    ///  //let rpc_server = Node::start_rpc_server(String::from("127.0.0.1"), 5001, Arc::new(Mutex::new(tx)));
    ///```
    pub fn start_rpc_server(bind_host: String, bind_port: u32, tx: Arc<Mutex<mpsc::Sender<Message>>>) {
        println!("Starting RPC server on {}:{}", bind_host, bind_port);
        let listener = TcpListener::bind(bind_host + ":" + &bind_port.to_string()).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let new_tx = tx.clone();
                    thread::spawn(move || {
                        Node::handle_client(stream, new_tx);
                    });
                }
                Err(_) => {
                    println!("Error");
                }
            }
        }
    }

    fn handle_client(mut stream: TcpStream, tx: Arc<Mutex<mpsc::Sender<Message>>>) {
        println!("New client connecting");
        loop {
            let mut read = [0; 1028];
            match stream.read(&mut read) {
                Ok(n) => {
                    if n == 0 { 
                        // connection was closed
                        break;
                    }
                    let deserialized = serde_json::from_slice(&read[0..n]);
                    match deserialized {
                        Ok(message) => {
                            match tx.lock() {
                                Ok(l) => {
                                    match l.send(message) {
                                        Ok(_) => {
                                            continue;
                                        },
                                        Err(e) => {
                                            println!("There was an error sending the deserialized message: {}", e);
                                            continue;
                                        },
                                    }
                                },
                                Err(e) => {
                                    println!("There was an error acquring lock on tx to send Message: {}", e);
                                    continue;
                                },
                            }
                        },
                        Err(e) => {
                            println!("There was an error deserializing an incoming message: {}", e);
                            continue;
                        },
                    };
                }
                Err(err) => {
                    panic!(err);
                }
            }
        }
    }

    /// Handles receiving a `Message`
    /// This is started in another thread in main, and it then loops in the background
    /// to receive messages on a channel.
    pub fn receive_message(mut self) {
        let rx_chan = self.config.rx.clone();
        loop {
            // TODO: These probably shouldn't just unwrap here
            let lock = rx_chan.lock().unwrap();
            let msg = lock.recv().unwrap();
            match msg.message_type {
                MessageType::HEARTBEAT => {
                    self.handle_heartbeat(&msg.args);
                },
                MessageType::REGISTER => {
                    self.handle_register(&msg.args);
                },
                MessageType::LIST_NODES => {
                    let nodes = self.handle_list_nodes(&msg.args);
                    let response = Message::new(MessageType::LIST_NODES).args(nodes);
                    msg.response_chan.unwrap().send(response);
                    continue;
                },
                MessageType::SHUTDOWN => {
                    println!("Shutting down...");
                    return;
                },
            };
        }
    }

    fn handle_list_nodes(&mut self, arguments: &Vec<String>) -> Vec<String> {
        MetadataDB::list_nodes(&self.db)
    }

    fn handle_heartbeat(&mut self, arguments: &Vec<String>) {

    }

    fn handle_register(&mut self, arguments: &Vec<String>) {
        println!("Received a registration request from: {:?}", arguments);
        match MetadataDB::register_node(&self.db, &arguments[0], &arguments[1], arguments[2].parse::<u16>().unwrap()) {
            true => {
                println!("Node registered!");
            },
            false => {
                println!("Node failed to register!");
            }
        }
    }

    /// Creates a Registration `Message` and sends it to the metadata server so we can join the cluster
    pub fn register_with_metadata_server(&mut self) -> Result<(), NodeError> {
        let serialized_message = Message::new(MessageType::REGISTER).args(
                vec![self.config.name.clone(), self.config.rpc_address.clone(), self.config.rpc_port.to_string()]
            ).to_vec()?;

        match self.metadata_connection {
            Some(ref mut conn) => {
                match conn.write(&serialized_message) {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(e) => {
                        Err(NodeError::new(&format!("Error writing to metadata conn: {}", e)))
                    },
                }
            },
            None => {
                match TcpStream::connect(self.config.metadata_address.clone() + ":" + &self.config.metadata_port.to_string()) {
                    Ok(conn) => {
                        println!("Connected to metadata server at: {}:{}", self.config.metadata_address, self.config.metadata_port);
                        self.metadata_connection = Some(conn);
                    },
                    Err(e) => {
                        println!("There was an error connecting to the metadata server: {:?}", e);
                        self.metadata_connection = None;
                    },
                };
                Err(NodeError::new("Error with connection to metadata server. Attempting reconnect..."))
            }
        }
    }
}

#[derive(Debug)]
pub struct NodeError {
    details: String
}

impl NodeError {
    /// Creates and returns a new DocumentError
    ///
    /// # Arguments
    ///
    /// * `msg` - The error message we want to include in the DocumentError
    ///
    /// # Example
    ///
    /// ```
    /// use rpc::node::NodeError;
    /// let _node_error = NodeError::new("Error with Node!");
    /// ```
    pub fn new(msg: &str) -> NodeError {
        NodeError { details: msg.to_string() }
    }
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NodeError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<serde_json::Error> for NodeError {
    fn from(err: serde_json::Error) -> NodeError {
        NodeError::new(&err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use node::{Node, NodeConfiguration};
    use messages::Message;
    use std::sync::{Arc, mpsc, Mutex};

    #[test]
    fn test_handle_heartbeat() {
        let (_my_node_tx, my_node_rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();

        let new_config = NodeConfiguration {
            name: String::from("test01"),
            metadata_address: String::from("localhost"),
            data_path: String::from("memory"),
            metadata_port: 5000,
            am_metadata_server: true,
            rx: Arc::new(Mutex::new(my_node_rx)),
            rpc_address: String::from("localhost"),
            rpc_port: 5001
        };

        let mut new_node = Node::new(new_config);
        new_node.handle_heartbeat(&vec![]);
    }

    #[test]
    fn test_handle_register() {
        let (_my_node_tx, my_node_rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();

        let new_config = NodeConfiguration {
            name: String::from("test02"),
            metadata_address: String::from("localhost"),
            data_path: String::from("memory"),
            metadata_port: 5000,
            am_metadata_server: true,
            rx: Arc::new(Mutex::new(my_node_rx)),
            rpc_address: String::from("localhost"),
            rpc_port: 5001
        };

        let mut new_node = Node::new(new_config);
        new_node.handle_register(&vec![]);
    }
}