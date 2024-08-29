use std::collections::HashMap;

use server::{base_handler, other_handler, Handler, IntoResponse, Request, Response, Router, Server, ServerError, ServerState};

#[derive(Debug)]
struct OuterHashmap(HashMap<String, String>);

impl IntoResponse for OuterHashmap {
    fn build(&self) -> server::Response {
        let body = format!("{:#?}", self.0);
        Response::new(http::StatusCode::OK, body)
    }
}

fn some_kinda_handler(req : Request) -> String {
    let mut hm = HashMap::new();
    hm.insert("hola".to_owned(), "hola".to_owned());
    hm.insert("chau".to_owned(), "chau".to_owned());
    return format!("{:#?}", hm);
}


fn main() -> Result<(), ServerError> {
    let router = Router::new().handler("/".into(), Handler(base_handler)).handler("/jsoncito".into(), Handler(some_kinda_handler));
    // .handler("/pepa".into(), Handler(other_handler));
    let srv = Server::bind("0.0.0.0:3000")?;
    if let ServerState::Connected(server) = srv {
        server.serve(router)?
    }
    Ok(())
}
