use hyper::{Request, Response, Body, Chunk};
use serde_json;
use futures::Stream;
use futures::Future;

use handlers::constants;

/// Handles the request to create a new index
pub fn create_index(req: Request) -> Response {
//    req.body().concat2()
//        .and_then(|body| {
//            let stringify = str::from_utf8(&body).unwrap();
//        });
    Response::new()
}

// Represents a request to create a new index. JSON should be
// de-serialized into one of these structs
#[derive(Serialize, Deserialize, Debug)]
struct CreateIndex {
    // Name of the index
    name: String,
    // Number of primary shards
    primary: i32,
    // Number of replica shards
    replica: i32
}