
use std::error::Error;
use std::fmt;

use futures::future::Future;
use regex::Regex;

use hyper::Method;
use hyper::server::{Request, Response};

type ResponseFuture = Response;
type Handler = fn(Request) -> ResponseFuture;

/// Router accepts components of an HTTP request and tries to find the appropriate function to handle it
pub struct Router {
    child_routers: Vec<Router>,
    routes: Vec<Route>,
}

/// Route is a combination of a regex, a HTTP Method, and a Handler function. When a request with a matching
/// regex and method come in, the Handler is executed.
pub struct Route {
    regex: Regex,
    verb: Method,
    handler: Handler
}

impl Route {
    /// Creates and returns a new Route
    /// 
    /// # Arguments
    /// 
    /// * `re` - A &str that will be compiled into a Regex
    /// * `verb` - The HTTP Method that should be matched
    /// 
    /// # Example
    /// 
    /// ```
    /// extern crate hyper;
    /// extern crate web;
    /// fn test(req: hyper::server::Request) -> hyper::server::Response {
    ///     hyper::server::Response::new()
    /// };
    /// let new_route = web::router::Route::new("/test", hyper::Method::Get, test);
    /// ```
    pub fn new(re: &str, verb: Method, handler: Handler) -> Result<Route, RouterError> {
        let re = Regex::new(re);
        if re.is_err() {
            return Err(RouterError::new("Invalid route"));
        }

        Ok(
            Route {
                regex: re.unwrap(),
                verb: verb,
                handler: handler
            }
        )
    }

    fn is_match(&self, verb: &Method, path: &str) -> Option<Handler> {
        if self.verb != *verb {
            return None;
        }

        if !self.regex.is_match(path) {
            return None;
        }

        return Some(self.handler);
    }


}

impl Router {
    /// Creates and returns a new Router
    /// A Router holds routes and child routers, and each request's method and path
    /// is matched against routes in each router until a match is found
    /// 
    /// # Example
    /// 
    /// ```
    /// extern crate web;
    /// let new_router = web::router::Router::new();
    /// ```
    pub fn new() -> Router {
        Router {
            child_routers: vec![],
            routes: vec![],
        }
    }

    /// Adds a Route to the Router
    /// 
    /// # Arguments
    /// 
    /// * `route` - The Route we want to add
    /// 
    /// # Example
    /// ```
    /// extern crate web;
    /// extern crate hyper;
    /// let mut new_router = web::router::Router::new();
    /// fn test(req: hyper::server::Request) -> hyper::server::Response {
    ///     hyper::server::Response::new()
    /// };
    /// let new_route = web::router::Route::new("/healthz", hyper::Method::Get, test).unwrap();
    /// new_router.add_route(new_route);
    /// ```
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Goes through each Route of a Router and tries to find one that matches
    /// 
    /// # Arguments
    /// 
    /// * `verb` - Reference to the HTTP Method in the request
    /// * `path` - The path string that was in the request
    /// 
    /// # Example
    /// ```
    /// extern crate web;
    /// extern crate hyper;
    /// let mut new_router = web::router::Router::new();
    /// fn test(req: hyper::server::Request) -> hyper::server::Response {
    ///     hyper::server::Response::new()
    /// };  
    /// let new_route = web::router::Route::new("/healthz", hyper::Method::Get, test).unwrap();
    /// new_router.add_route(new_route);
    /// let _result = new_router.route(&hyper::Method::Get, "/healthz");
    /// ```
    pub fn route(&self, verb: &Method, path: &str) -> Option<Handler> {
        for route in &self.routes {
            match route.is_match(verb, path) {
                Some(h) => {
                    return Some(h)
                },
                None => {}
            };
        }

        None
    }


    /// Adds a child Router. If a Router has children, and none of its
    /// routes match, searching will continue down into the children.
    /// 
    /// # Arguments
    /// 
    /// * `child` - Another instance of Router
    /// 
    /// # Example
    /// 
    /// ```
    /// extern crate web;
    /// let mut new_router1 = web::router::Router::new();
    /// let new_router2 = web::router::Router::new();
    /// new_router1.add_child(new_router2);
    /// ```
    pub fn add_child(mut self, child: Router) -> Router {
        self.child_routers.push(child);
        self
    }
}

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RouterError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
/// Custom error type returned when there is an error relating to Routers or
/// routes.
pub struct RouterError {
    details: String,
}

impl RouterError {
    /// Creates and returns a new RouterError
    ///
    /// # Arguments
    ///
    /// * `msg` - The error message we want to include in the RouterError
    ///
    /// # Example
    ///
    /// ```
    /// use web::router::RouterError;
    /// let router_error = RouterError::new("There was an error routing the request!");
    /// ```
    pub fn new(msg: &str) -> RouterError {
        RouterError { details: msg.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_router() {
        let router = Router::new();
        assert_eq!(router.child_routers.len(), 0);
    }

    #[test]
    fn test_create_route() {
        fn test(req: Request) -> Response {
            Response::new()
        };
        let route = Route::new("/healthz", Method::Get, test).unwrap();
        assert_eq!(route.verb, Method::Get);
    }

    #[test]
    fn test_route_matches() {
        fn test(req: Request) -> Response {
            Response::new()
        };
        let mut router = Router::new();
        let route = Route::new("/hea.*", Method::Get, test).unwrap();
        router.add_route(route);
        let result = router.route(&Method::Get, "/healthz");
        assert!(result.is_some());
    }
}
