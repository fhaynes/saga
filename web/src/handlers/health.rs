use std::sync::{Arc, Mutex};

use hyper::server::{Request, Response};
use hyper::StatusCode;

use rpc::Switchboard;

/// A basic health-check function. Returns 200 OK if a request can reach it
pub fn health_check(req: Request, swb: Arc<Mutex<Switchboard>>) -> Response {
    Response::new().with_status(StatusCode::Ok)
}