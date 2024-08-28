use crate::core::request::Request;
use crate::core::response::Response;
use std::collections::HashMap;

type Handler = fn(Request) -> Response;
pub struct Router {
    pub(crate) routes: HashMap<String, Handler>,
}

impl Router {
    pub fn route(&mut self, request: &Request) -> Option<&mut Handler> {
        let path = &request.path;
        return self.routes.get_mut(path);
    }
    pub fn new() -> Self {
        Router {
            routes: HashMap::<String, Handler>::new(),
        }
    }
    pub fn handler(mut self, path: String, route: Handler) -> Self {
        self.routes.insert(path, route);
        return self;
    }
}
