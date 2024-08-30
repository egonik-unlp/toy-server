use std::collections::HashMap;

use server::{
    base_handler, handlers::get, other_handler, Handler, Request, Response, ResponseBody, Router,
    Server, ServerError, ServerState,
};
#[derive(Debug)]
struct OuterHashmap(HashMap<String, String>);

impl Into<ResponseBody> for OuterHashmap {
    fn into(self) -> ResponseBody {
        return ResponseBody {
            content: format!("{:#?}", self.0),
        };
    }
}

fn some_kinda_handler(req: &Request) -> OuterHashmap {
    let mut hm = HashMap::new();
    hm.insert("hola".to_owned(), "hola".to_owned());
    hm.insert("chau".to_owned(), "chau".to_owned());
    return OuterHashmap(hm);
}

fn main() -> Result<(), ServerError> {
    let router = Router::new()
        .handler("/".into(), base_handler)
        .handler("/jsoncito".into(), some_kinda_handler)
        .handler("/pepa".into(), other_handler);
    let srv = Server::bind("0.0.0.0:3000")?;
    if let ServerState::Connected(server) = srv {
        server.serve(router)?
    }
    Ok(())
}
