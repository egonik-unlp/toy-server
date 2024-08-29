use http::StatusCode;

use crate::core::request::Request;
use crate::core::response::Response;
use std::collections::HashMap;
use std::fmt::Debug;

pub trait IntoResponse: Debug {
    fn build(&self) -> Response;
}

impl IntoResponse for String {
    fn build(&self) -> Response {
        Response::new(StatusCode::OK, self.into())
    }
}
// type Handler<R: ToString + Sized + 'static> = fn(Request) -> R;
pub struct Router<R>
where
    R: IntoResponse,
{
    pub(crate) routes: HashMap<String, Handler<R>>,
}
#[derive(Debug)]
pub struct Handler<R>(pub fn(Request) -> R);

impl<R> Router<R>
where
    R: IntoResponse,
{
    pub(crate) fn route(&mut self, request: &Request) -> Option<&mut Handler<R>> {
        let path = &request.path;
        return self.routes.get_mut(path);
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
