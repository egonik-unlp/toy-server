use http::StatusCode;

use crate::core::request::Request;
use crate::core::response::IntoResponse;
use std::collections::HashMap;
use std::fmt::Debug;

use super::request::RequestError;
use super::response::ResponseBody;

// type Handler<R: ToString + Sized + 'static> = fn(Request) -> R;
pub struct Router {
    pub(crate) routes: HashMap<String, Box<dyn Handler>>,
}
// #[derive(Debug)]
// pub struct Handler(pub fn(Request) -> Box<dyn IntoResponse>);

pub mod handlers {
    use http::StatusCode;

    use crate::Response;

    use super::{Handler, Request};
    pub fn get(handler: impl Handler) -> Response {
        return Response::new(StatusCode::OK, "a".to_owned(), "text/plain".into());
    }
}

pub trait Handler {
    fn handle(&self, request: &Request) -> ResponseBody;
}

impl<F, R> Handler for F
where
    R: IntoResponse,
{
    fn handle(&self, request: &Request) -> ResponseBody {
        let resp_body = self(&request);
        return resp_body.into();
    }
}

impl Into<ResponseBody> for String {
    fn into(self) -> ResponseBody {
        return ResponseBody { content: self , content_type: "text/plain".into()};
    }
}

impl Router {
    pub(crate) fn route(&mut self, request: &Request) -> Result<&mut Box<dyn Handler>, RequestError> {
        let path = &request.path;
        if path.contains("<") {
            
        }
        let route = self.routes.get_mut(path).ok_or_else(|_| RequestError {inner : "No handler for path {}", r});
        return Ok;
    }
    pub fn new() -> Self {
        Router {
            routes: HashMap::<String, Handler<R>>::new(),
        }
    }
    pub fn handler(mut self, path: String, route: Handler<R>) -> Self {
        self.routes.insert(path, route);
        return self;
    }
}
