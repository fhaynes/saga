#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate regex;
extern crate rusqlite;
extern crate rpc; 

pub mod handlers;
pub mod router;

use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use futures::future::Future;

use hyper::server::{Request, Response, Service};
use hyper::StatusCode;

use rpc::messages::Message;

pub struct Saga {
    pub router: router::Router,
    pub config: ServiceConfiguration
}

impl Service for Saga {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match self.router.route(req.method(), req.path()) {
            Some(h) => {
                Box::new(futures::future::ok(
                    h(req, self.config.switchboard.clone())
                ))
            },
            None => {
                Box::new(futures::future::ok(
                    Response::new().with_status(StatusCode::NotFound)
                ))
            }
        }
    }
}

pub struct ServiceConfiguration {
    pub data_path: String,
    pub switchboard: Arc<Mutex<rpc::Switchboard>>,
}

impl ServiceConfiguration {
    pub fn new(data_path: String, switchboard: Arc<Mutex<rpc::Switchboard>>) -> ServiceConfiguration {
        ServiceConfiguration {
            data_path: data_path,
            switchboard: switchboard
        }
    }
}