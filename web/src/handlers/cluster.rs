use std::sync::{Arc, Mutex, mpsc};
use hyper::server::{Request, Response};
use hyper::header::ContentType;
use rusqlite::Connection;
use serde_json;

use rpc::Switchboard;
use rpc::messages::{MessageType, Message};

/// Handles a request to list all nodes in the cluster
pub fn list_nodes(req: Request, swb: Arc<Mutex<Switchboard>>) -> Response {
    let (resp_tx, resp_rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    let query = Message::new(MessageType::LIST_NODES).response_chan(resp_tx);
    let node_tx = swb.lock().unwrap().node_tx.clone();
    node_tx.lock().unwrap().send(query);
    let response = resp_rx.recv().unwrap();
    let json_response = serde_json::to_string(&response.args).unwrap();
    Response::new().with_body(json_response)
}

/// Handles a request from a data node to register
/// Only used if we are the metadata node
pub fn register(req: Request, swb: Arc<Mutex<Switchboard>>) -> Response {
    Response::new()
}

