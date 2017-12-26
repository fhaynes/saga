use std::sync::{Arc, Mutex};
use hyper::server::{Request, Response};
use rpc::Switchboard;

pub fn health_check(req: Request, swb: Arc<Mutex<Switchboard>>) -> Response {
    Response::new()
}