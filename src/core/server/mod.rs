use crate::core::request::Request;
use crate::core::response::Response;
use crate::core::router::Router;
use http::StatusCode;
use std::fmt::Debug;
use std::io::{BufReader, Error, Read, Write};
use std::net::TcpListener;
use std::time::Duration;

use super::response::IntoResponse;

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
    pub fn serve<R>(&self, mut router: Router<R>) -> Result<(), ServerError>
    where
        R: IntoResponse,
    {
        println!(
            "Serving requests on port {}",
            self.connection.local_addr().unwrap()
        );
        println!("Routes defined\n{:#?}", router.routes);

        for stream in self.connection.incoming() {
            let mut st = stream.map_err(|err| ServerError::new(err))?;
            st.set_read_timeout(Some(Duration::from_millis(500)))
                .map_err(|err| ServerError::new(err))?;
            let mut buffered_stream = BufReader::new(st);
            let response = match Request::new(&mut buffered_stream) {
                Err(err) => Response::new(StatusCode::INTERNAL_SERVER_ERROR, err.inner),
                Ok(req) => match router.route(&req) {
                    Some(handler) => {
                        println!("request: {:?}\nhandler:{:?}\n", req, handler);
                        handler.0(req).build()
                    },
                    None => Response::new(
                        StatusCode::NOT_FOUND,
                        format!("resource {} not found", req.path),
                    ),
                },
            };
            println!("{:?}", response);
            response.respond(buffered_stream.get_mut()).map_err(|err| detector(err))?;
        }
        Ok(())
    }
}
