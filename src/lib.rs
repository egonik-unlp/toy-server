use http::status::StatusCode;
mod core;
pub use crate::core::{
    request::Request,
    response::Response,
    router::Router,
    server::{Server, ServerError, ServerState},
};

pub fn base_handler(request: Request) -> Response {
    Response::new(StatusCode::OK, format!("{:#?}", request))
}

pub fn other_handler(_request: Request) -> Response {
    let rsp = Response::new(StatusCode::OK, "HELLOOOO".into());
    Response::new(StatusCode::OK, format!("{}", rsp))
}
