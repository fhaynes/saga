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

// Imports
use std::sync::{Arc,Mutex};
use futures::future::Future;

use hyper::server::{Request, Response, Service};
use hyper::StatusCode;

/// Saga is a struct that will be used to implement the Hyper Service Trait
pub struct Saga {
    /// The `Router` the `Service` will use to find `Handlers` for `Requests`
    pub router: router::Router,
    /// Convenience struct that holds configuration variables used by the Service
    pub config: ServiceConfiguration
}

impl Service for Saga {
    /// Next four lines are boilerplate for a Hyper service
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    /// This called for every web request the server receives
    fn call(&self, req: Request) -> Self::Future {
        match self.router.route(req.method(), req.path()) {
            Some(h) => {
                Box::new(futures::future::ok(
                    h(req, self.config.switchboard.clone())
                ))
            },
            None => {
                // If no matching Handler is found, return NotFound
                Box::new(futures::future::ok(
                    Response::new().with_status(StatusCode::NotFound)
                ))
            }
        }
    }
}

/// Convenience type to hold configuration variables used with the Hyper Service
pub struct ServiceConfiguration {
    /// Location of the root data path
    pub data_path: String,
    /// A `Switchboard` struct so we can easily send messages to other parts of the application
    pub switchboard: Arc<Mutex<rpc::Switchboard>>,
}

impl ServiceConfiguration {
    /// Creates and returns a ServiceConfiguration
    pub fn new(data_path: String, switchboard: Arc<Mutex<rpc::Switchboard>>) -> ServiceConfiguration {
        ServiceConfiguration {
            data_path: data_path,
            switchboard: switchboard
        }
    }
}