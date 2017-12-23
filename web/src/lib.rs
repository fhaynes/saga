#![feature(plugin, use_extern_macros)]
#![plugin(tarpc_plugins)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate tarpc;

extern crate futures;
extern crate hyper;
extern crate regex;

pub mod handlers;
pub mod router;


use futures::future::Future;

use hyper::server::{Request, Response, Service};
use hyper::StatusCode;

pub struct Saga {
    pub router: router::Router
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
                    h(req)
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

