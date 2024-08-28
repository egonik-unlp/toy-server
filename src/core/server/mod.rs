use crate::core::request::Request;
use crate::core::response::Response;
use crate::core::router::Router;
use http::StatusCode;
use std::io::Error;
use std::net::TcpListener;
use std::time::Duration;

pub enum ServerState {
    Connected(ConnectedServer),
    Disconnected,
}

pub struct ConnectedServer {
    connection: TcpListener,
}

pub struct Server {
    state: ServerState,
}

#[derive(Debug)]
pub struct ServerError {
    inner: String,
}

impl ServerError {
    fn new(error: Error) -> Self {
        return ServerError {
            inner: format!("Error creating server :{}", error),
        };
    }
}
impl Default for Server {
    fn default() -> Self {
        return Self {
            state: ServerState::Disconnected,
        };
    }
}

impl Server {
    pub fn bind(adress: &str) -> Result<ServerState, ServerError> {
        let listener = TcpListener::bind(adress).map_err(|err| ServerError::new(err))?;
        let server = ServerState::Connected(ConnectedServer {
            connection: listener,
        });
        return Ok(server);
    }
}
fn detector(err: Error) -> ServerError {
    return ServerError {
        inner: format!("vino de por aqui? {}", err),
    };
}

impl ConnectedServer {
    pub fn serve(&self, mut router: Router) -> Result<(), ServerError> {
        println!(
            "Serving requests on port {}",
            self.connection.local_addr().unwrap()
        );
        println!("Routes defined\n{:#?}", router.routes);

        for stream in self.connection.incoming() {
            let mut st = stream.map_err(|err| ServerError::new(err))?;
            st.set_read_timeout(Some(Duration::from_secs(1)))
                .map_err(|err| ServerError::new(err))?;
            let response = match Request::new(&mut st) {
                Err(err) => Response::new(StatusCode::INTERNAL_SERVER_ERROR, err.inner),
                Ok(req) => match router.route(&req) {
                    Some(handler) => handler(req),
                    None => Response::new(
                        StatusCode::NOT_FOUND,
                        format!("resource {} not found", req.path),
                    ),
                },
            };

            response.respond(&mut st).map_err(|err| detector(err))?;
        }
        Ok(())
    }
}
