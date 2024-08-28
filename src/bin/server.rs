use server::{base_handler, other_handler, Router, Server, ServerError, ServerState};

fn main() -> Result<(), ServerError> {
    let router = Router::new()
        .handler("/".into(), base_handler)
        .handler("/pepa".into(), other_handler);
    let srv = Server::bind("0.0.0.0:3000")?;
    if let ServerState::Connected(server) = srv {
        server.serve(router)?
    }
    Ok(())
}
