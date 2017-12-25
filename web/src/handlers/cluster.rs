use hyper::server::{Request, Response};

use rusqlite::Connection;

/// Handles a request to list all nodes in the cluster
pub fn list_nodes(req: Request) -> Response {
    //let conn = Connection::open
    //let nodes = rpc::MetadataDB::list_nodes();
    Response::new()
}

/// Handles a request from a data node to register
/// Only used if we are the metadata node
pub fn register(req: Request) -> Response {
    Response::new()
}

