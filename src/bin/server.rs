use std::collections::HashMap;

use server::{base_handler, other_handler, Handler, IntoResponse, Request, Response, Router, Server, ServerError, ServerState};

#[derive(Debug)]
struct OuterHashmap(HashMap<String, String>);

impl Into<ResponseBody> for OuterHashmap {
    fn into(self) -> ResponseBody {
        return ResponseBody {
            content: format!("{:#?}", self.0),
            content_type: "application/json".into()
        };
    }
}

fn dangerous_handler(req: &Request) -> String {
    return req.path.rsplit_once("/").unwrap().1.to_owned();

}



fn some_kinda_handler(req: &Request) -> OuterHashmap {
    let mut hm = HashMap::new();
    hm.insert("hola".to_owned(), "hola".to_owned());
    hm.insert("chau".to_owned(), "chau".to_owned());
    return format!("{:#?}", hm);
}


fn main() -> Result<(), ServerError> {
    let address = match std::env::args().nth(1)  {
        Some(port) => {
            let _ = port.parse::<i32>().map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Ivalid port"));
            println!("HAI");
            format!("0.0.0.0:{}",port)
    },
        None => {
            let falopa = "0.0.0.0:3000";
            println!("As no port is provided via env args, using default {}", falopa);
            falopa.into()
            
        }
    };
    let router = Router::new()
        .handler("/".into(), base_handler)
        .handler("/jsoncito".into(), some_kinda_handler)
        .handler("/pepa".into(), other_handler)
        .handler("/text/index.html".into(), dangerous_handler);
    let srv = Server::bind(&address)?;
    if let ServerState::Connected(server) = srv {
        server.serve(router)?
    }
    Ok(())
}
