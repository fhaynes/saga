use std::sync::{Arc, Mutex, mpsc};

use hyper::server::{Request, Response};
use hyper::StatusCode;
use serde_json;

use rpc::Switchboard;
use rpc::messages::{MessageType, Message};

/// Handles a request to list all nodes in the cluster
pub fn list_nodes(_req: Request, swb: Arc<Mutex<Switchboard>>) -> Response {
    let (resp_tx, resp_rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    let query = Message::new(MessageType::LIST_NODES).response_chan(resp_tx);
    let node_tx = match swb.lock() {
        Ok(l) => {
            l.node_tx.clone()
        },
        Err(e) => {
            return Response::new().with_status(StatusCode::InternalServerError);
        },
    };

    match node_tx.lock() {
        Ok(l) => {
            l.send(query);
        },
        Err(e) => {
            return Response::new().with_status(StatusCode::InternalServerError);
        },
    };

    match resp_rx.recv() {
        Ok(response) => {
                match serde_json::to_string(&response.args) {
                    Ok(serialized) => {
                        return Response::new().with_body(serialized);
                    },
                    Err(e) => {
                        return Response::new().with_status(StatusCode::InternalServerError);            
                    },
                };
        },
        Err(e) => {
            return Response::new().with_status(StatusCode::InternalServerError);
        }
    };
}

/// Handles a request from a data node to register
/// Only used if we are the metadata node
pub fn register(req: Request, swb: Arc<Mutex<Switchboard>>) -> Response {
    Response::new()
}

