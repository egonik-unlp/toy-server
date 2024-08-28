use std::io::Error;
use std::io::Read;
use std::net::TcpStream;
use std::str::FromStr;

#[derive(Debug)]
enum RequestHeaderType {
    Host,
    Accept,
    AcceptLanguage,
    AcceptEncoding,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub(crate) path: String,
    version: String,
}
#[derive(Debug)]
pub struct ConnectionError {
    inner: String,
}

#[derive(Debug)]
pub(crate) struct RequestError {
    pub(crate) inner: String,
}
impl RequestError {
    fn new(msg: String) -> Self {
        return RequestError { inner: msg };
    }
}

impl Request {
    pub(crate) fn new(stream: &mut TcpStream) -> Result<Self, RequestError> {
        parse_request_arguments(stream)
            .map_err(|err| RequestError::new(format!("Error processing headers: {}", err)))
    }
}
fn parse_request_arguments(stream: &mut TcpStream) -> Result<Request, std::io::Error> {
    let header = read_first_line(stream)?;
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
