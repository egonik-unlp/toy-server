use http::status::StatusCode;
use httpdate::fmt_http_date;
use std::{
    collections::HashMap,
    fmt::{format, Debug},
    hash::Hash,
    io::{Error, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    path::Display,
    str::FromStr,
    time::{Duration, SystemTime},
};
#[derive(Debug, Clone, Copy)]
enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl FromStr for Method {
    type Err = RequestError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            _ => Err(RequestError {
                inner: "Method not recognised".to_owned(),
            }),
        };
    }
}

pub struct ResponseBody {
    content: String,
}

pub trait IntoResponse {
    fn build_response_body(self) -> ResponseBody;
}

pub struct Response {
    code: StatusCode,
    body: String,
    // headers: Hesaders,
}
impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {}
            Date : {}
            Server : {}
            Content-Length : {}
            Content-Type : {}

            {}
            ",
            self.code,
            fmt_http_date(SystemTime::now()),
            "ServerEdu",
            self.body.len(),
            "text/plain",
            self.body
        )
    }
}

impl Response {
    pub fn respond(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        return write!(stream, "{}", self);
    }
    pub fn new(code: StatusCode, body: String) -> Self {
        // let mut headers_for_now = HashMap::new();
        // headers_for_now.insert(ResponseHeaderType::Server, "EduServer".into());
        return Response {
            code: code,
            body: body,
            // headers: Headers(headers_for_now),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ResponseHeaderType {
    ContentLength,
    Date,
    ContentType,
    Server,
    ContentEncoding,
}
#[derive(Debug)]
enum RequestHeaderType {
    Host,
    Accept,
    AcceptLanguage,
    AcceptEncoding,
}

impl IntoResponse for String {
    fn build_response_body(self) -> ResponseBody {
        return ResponseBody { content: self };
    }
}

#[derive(Debug)]
struct Headers(HashMap<ResponseHeaderType, String>);

pub enum ServerState {
    Connected(ConnectedServer),
    Disconnected,
}

pub struct ConnectedServer {
    connection: TcpListener,
}

struct Server {
    state: ServerState,
}

#[derive(Debug)]
struct ServerError {
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

impl ConnectedServer {
    pub fn serve(&self, mut router: Router) -> Result<(), ServerError> {
        for stream in self.connection.incoming() {
            let mut st = stream.map_err(|err| ServerError::new(err))?;
            st.set_read_timeout(Some(Duration::from_secs(1)));
            let request = Request::new(&mut st).unwrap();
            let response = match router.route(&request) {
                Some(handler) => handler(request),
                None => Response::new(StatusCode::NOT_FOUND, "NOT FOUND".into()),
            };
            response
                .respond(&mut st)
                .map_err(|err| ServerError::new(err))?;
            st.shutdown(Shutdown::Write)
                .map_err(|err| ServerError::new(err))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    method: String,
    path: String,
    version: String,
}
#[derive(Debug)]
struct RequestError {
    inner: String,
}
impl RequestError {
    fn new(msg: String) -> Self {
        return RequestError { inner: msg };
    }
}

impl Request {
    fn new(stream: &mut TcpStream) -> Result<Self, RequestError> {
        parse_request_arguments(stream)
            .map_err(|err| RequestError::new(format!("Error processing headers: {}", err)))
    }
}

// type Handler = Box<dyn FnMut(Request) -> Response>;
type Handler = fn(Request) -> Response;
pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    pub fn route(&mut self, request: &Request) -> Option<&mut Handler> {
        let path = &request.path;
        return self.routes.get_mut(path);
    }
    pub fn new() -> Self {
        Router {
            routes: HashMap::<String, Handler>::new(),
        }
    }
    pub fn handler(&mut self, path: String, route: Handler) {
        self.routes.insert(path, route);
    }
}

fn parse_request_arguments(stream: &mut TcpStream) -> Result<Request, std::io::Error> {
    let header = read_first_line(stream).unwrap();
    let mut header_iter = header.split_whitespace();
    let method = header_iter.next().ok_or(Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid request header format. Method not found",
    ))?;
    let path = header_iter.next().ok_or(Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid request header format. Path not found",
    ))?;
    let version = header_iter.next().ok_or(Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid request header format. HTTP Version not found",
    ))?;
    let request = Request {
        method: method.to_owned(),
        path: path.to_owned(),
        version: version.to_owned(),
    };
    return Ok(request);
}

fn read_first_line(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let mut buffer = Vec::with_capacity(4096);
    while let Some(Ok(byte)) = stream.bytes().next() {
        if byte.eq(&b'\n') {
            return String::from_utf8(buffer)
                .map_err(|_| Error::new(std::io::ErrorKind::ConnectionAborted, "incomplete data"));
        } else {
            buffer.push(byte)
        }
    }
    let error = std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid request data");
    return Err(error);
}

fn base_handler(request: Request) -> Response {
    Response::new(StatusCode::OK, "HELLOOOO".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();
    router.handler("/".into(), base_handler);
    let srv = Server::bind("0.0.0.0:3000").unwrap();
    match srv {
        ServerState::Connected(server) => server.serve(router).unwrap(),
        ServerState::Disconnected => (),
    }
    Ok(())
}
