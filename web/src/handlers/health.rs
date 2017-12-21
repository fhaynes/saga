use hyper::server::{Request, Response};

pub fn health_check(req: Request) -> Response {
    Response::new()
}