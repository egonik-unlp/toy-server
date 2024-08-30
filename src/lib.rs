// use core::response::IntoResponse;

use http::status::StatusCode;
mod core;
pub use crate::core::{
    request::Request,
    response::{Response,ResponseBody},
    router::{handlers, Handler, Router},
    server::{Server, ServerError, ServerState},
};

pub fn base_handler(request: Request) -> String {
    // Response::new(StatusCode::OK, format!("{:#?}", request))
    return "HOLA".into();
}

pub fn other_handler(_request: Request) -> Response {
    let rsp = Response::new(StatusCode::OK, "HELLOOOO".into());
    Response::new(StatusCode::OK, format!("{}", rsp))
}
