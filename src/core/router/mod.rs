
use async_trait::async_trait;
use http::{request, StatusCode};
use tokio::sync::RwLock;

use crate::core::request::Request;
use crate::Response;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::process::Output;
use std::sync::Arc;

use super::response::ResponseBody;

// type Handler<R: ToString + Sized + 'static> = fn(Request) -> R;
pub struct Router {
    pub(crate) routes: HashMap<String, Arc<RwLock<dyn Handler + Send + Sync>>>,
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
#[async_trait]
pub trait Handler:  {
    async fn handle(&self, request: &Request) -> ResponseBody;
}


#[async_trait]
impl<F, R, C> Handler for F
where
    C: Future<Output = R> + Sized + Send,
    R: Into<ResponseBody>,
    F: Fn(&Request) -> C + Sync ,
{
     async fn handle(&self, request: &Request) -> ResponseBody {
            let resp_body = self(&request).await;
            return resp_body.into();

    }
}


impl From<String> for ResponseBody {
    fn from(value: String) -> Self {
        return ResponseBody {
            content: value,
            content_type: "text/plain".into()
        };
    }
}




impl Router {
    pub(crate) fn route(&mut self, request: &Request) -> Option<&mut Arc<RwLock<dyn Handler + Send + Sync>>> {
        let path = &request.path;
        return self.routes.get_mut(path);
    }
    pub fn new() -> Self {
        Router {
            routes: HashMap::<String, Arc<RwLock<dyn Handler + Send + Sync>>>::new(),
        }
    }
    pub fn handler(mut self, path: String, route: impl Handler + 'static + Send + Sync) -> Self {
        self.routes.insert(path, Arc::new(RwLock::new(route)));
        return self;
    }
}
