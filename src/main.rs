use http::status::StatusCode;
use std::{
    collections::HashMap,
    fmt::{format, Debug},
    io::{Error, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    str::FromStr,
    time::Duration,
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

struct ResponseBody {
    content: String,
}

pub trait IntoResponseBody {
    fn build_response_body(self) -> ResponseBody;
}

struct Response<R: IntoResponseBody> {
    code: StatusCode,
    body: R,
    headers: Headers,
}

#[derive(Debug)]
enum HeaderType {
    ContentLength,
    Date,
    ContentType,
    Server,
    ContentEncoding,
}

impl IntoResponseBody for String {
    fn build_response_body(self) -> ResponseBody {
        return ResponseBody { content: self };
    }
}

#[derive(Debug)]
struct Headers(HashMap<HeaderType, String>);

fn write_handling(mut stream: TcpStream, payload: impl Debug) -> Result<(), std::io::Error> {
    let body = format!("{:#?}", payload);
    let header = format!(
        "HTTP/1.1 200 OK
    Date: Mon, 27 Jul 2009 12:28:53 GMT
    Server: Apache/2.2.14 (Win32)
    Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT
    Content-Length: {}
    Content-Type: text/html
    Connection: Closed\n\n{}",
        body.len(),
        body
    );
    println!("{}", header);
    let _b = stream.write(header.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

enum ServerState {
    Connected { connection: TcpListener },
    Disconnected,
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
    pub fn bind(adress: &str) -> Result<Self, ServerError> {
        let listener = TcpListener::bind(adress).map_err(|err| ServerError::new(err))?;
        let server = Server {
            state: ServerState::Connected {
                connection: listener,
            },
        };
        return Ok(server);
    }
}

#[derive(Debug, Clone)]
struct Request {
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
        let request = parse_request_arguments(stream)
            .map_err(|err| RequestError::new(format!("Error processing headers: {}", err)))?;

        todo!()
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
fn parse_request_headers(stream: &mut TcpStream) {
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let srv = TcpListener::bind("0.0.0.0:3210").unwrap();

    for stream in srv.incoming() {
        let mut stream = stream?;
        stream.set_read_timeout(Some(Duration::from_secs(1)))?;
        let request = Request::new(&mut stream)?;
        write_handling(stream, request)?;
    }
    Ok(())
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
