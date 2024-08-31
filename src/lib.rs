
use http::status::StatusCode;
mod core;
pub use crate::core::{
    request::Request,
    response::{Response, ResponseBody},
    router::{handlers, Handler, Router},
    server::{Server, ServerError, ServerState},
};

 pub async  fn base_handler(_request: &Request) -> String {
    // Response::new(StatusCode::OK, format!("{:#?}", request))
    return "HOLA".into();
}

pub fn other_handler(_request: &Request) -> String {
    let rsp = Response::new(StatusCode::OK, "HELLOOOO".into(), "text/plain".into());
    return format!("{}", rsp);
}
