use http::{request, StatusCode};

use crate::core::request::Request;
use crate::Response;
use std::collections::HashMap;
use std::fmt::Debug;

use super::response::ResponseBody;

// type Handler<R: ToString + Sized + 'static> = fn(Request) -> R;
pub struct Router{
    pub(crate) routes: HashMap<String, Box<dyn Handler>>,
}
// #[derive(Debug)]
// pub struct Handler(pub fn(Request) -> Box<dyn IntoResponse>);

pub mod handlers {
    use http::StatusCode;

    use crate::Response;

    use super::{Handler, Request};
    pub fn get(handler: impl Handler) -> Response {
        return Response::new(StatusCode::OK, "a".to_owned());
    }
}

pub trait Handler {
    fn handle(&self, request: &Request) -> ResponseBody;
}

impl<F,R> Handler for F 
where R: Into<ResponseBody>,
F: Fn(Request) -> R + Debug
{
    fn handle( &self, request: &Request) ->  ResponseBody {
        let resp_body = self(request.clone());
        return resp_body.into();
    }
}




impl Router {
    pub(crate) fn route(&mut self, request: &Request) -> Option<&mut Box<dyn Handler>> {
        let path = &request.path;
        return self.routes.get_mut(path)
    }
    pub fn new() -> Self {
        Router {
            routes: HashMap::<String, Box<dyn Handler>>::new(),
        }
    }
    pub fn handler(mut self, path: String, route: impl Handler+ 'static ) -> Self {
        self.routes.insert(path, Box::new(route));
        return self;
    }
}
