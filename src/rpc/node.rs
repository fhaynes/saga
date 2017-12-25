use rpc::messages::{Message, MessageType};

use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs;

use std::sync::{Arc, Mutex, mpsc};
use std::path::Path;

use rusqlite::Connection;
use serde_json;

/// Node is an individual server within a Cluster
pub struct Node {
    ip: String,
    rpc_port: u16,
    metadata_server: bool,
    rx: mpsc::Receiver<Message>,
    pub db: Connection
}

impl Node {
    pub fn new<S: Into<String>>(ip: S, rpc_port: u16, metadata_server: bool, rx: mpsc::Receiver<Message>, data_path: &str) -> Node {
        let path = data_path.to_string() + "metadata/";
        let result = fs::create_dir_all(&path);
        let metadata_db: Connection;
        if result.is_err() {
            println!("Unable to create the director for the metadata database. It will be created in memory.");
            metadata_db = Connection::open_in_memory().unwrap();
        } else {
            metadata_db = Connection::open(&path).unwrap();
        }

        Node {
            ip: ip.into(),
            rpc_port: rpc_port,
            metadata_server: metadata_server,
            rx: rx,
            db: metadata_db,
        }
    }

    pub fn start_rpc_server<S: Into<String>>(bind_host: S, bind_port: u32, tx: mpsc::Sender<Message>) {
        println!("Starting RPC server...");
        let listener = TcpListener::bind(bind_host.into() + ":" + &bind_port.to_string()).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let new_tx = Arc::new(Mutex::new(tx.clone()));
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
                                        Ok(r) => {
                                            continue;
                                        },
                                        Err(e) => {
                                            println!("There was an error sending the deserialized message");
                                            continue;
                                        },
                                    }
                                },
                                Err(e) => {
                                    println!("There was an error acquring lock on tx to send Message");
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

    pub fn receive_message(&mut self) {
        loop {
            let msg = self.rx.recv();
            if msg.is_err() {
                println!("Error receiving RPC message!");
                continue;
            }
            let msg = msg.unwrap();
            match msg.message_type {
                MessageType::HEARTBEAT => {
                    println!("Received heartbeat message");
                },
                MessageType::REGISTER => {
                    println!("Received register message");
                },
            }
        }
    }

    fn handle_heartbeat(arguments: &Vec<String>) {

    }

    fn handle_register(arguments: &Vec<String>) {
        
    }
}