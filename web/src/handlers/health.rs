use hyper::server::{Request, Response};

pub fn health_check(req: Request) -> Response {
    println!("Beginning health check handler");
    Response::new()
}