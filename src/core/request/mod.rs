use std::collections::HashMap;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use std::net::TcpStream;
use std::str::FromStr;

use http::request;
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
    pub(crate) headers: Headers
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
    pub(crate) fn new(stream:&mut BufReader<TcpStream>) -> Result<Self, RequestError> {
       let (method, path, version) = parse_request_arguments(stream).map_err(|err| RequestError::new(format!("Error processing headers: {}", err)))?;
       let mut buf_remainig = Vec::with_capacity(4096);
       stream.read(&mut buf_remainig).unwrap();
       println!("{}", String::from_utf8(buf_remainig).unwrap());
       let headers = parse_headers(stream).map_err(|err| RequestError::new(format!("Error processing headers: {:?}", err)))?;
       return Ok( Request { method: method, path: path, version: version, headers: headers })

    }
}

type Headers = HashMap<String, Vec<String>>;

fn parse_headers(stream: &mut BufReader<TcpStream>) -> Result<Headers, RequestError> {
    let mut headers = Headers::new();
    loop {
        let line = read_first_line(stream).map_err(|err| RequestError::new(format!("Error parseando headers: {}",err )))?;
        if line.len().eq(&0) {
             break;
            }
        let mut parts = line.split(": ");
        let header_name = parts.next().ok_or_else(|| RequestError::new("Error parseando nombre de header".into()))?;
        let content = parts.next().ok_or_else(|| RequestError::new("Error parseando valor de header".into()))?;
        let slot_for_value = headers
                .entry(header_name.to_owned())
                .or_insert_with(|| { Vec::with_capacity(1) });
            slot_for_value.push(content.to_owned());
    }   
 
    return Ok(headers) ;
 
}


fn parse_request_arguments(stream: &mut BufReader<TcpStream>) -> Result<(String, String,String), std::io::Error> {
    let header = read_first_line(stream)?;
    let mut header_iter = header.split_whitespace();
    let method = header_iter.next().ok_or(Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid request header format. Method not found",
    ))?.to_owned();
    let path = header_iter.next().ok_or(Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid request header format. Path not found",
    ))?.to_owned();
    let version = header_iter.next().ok_or(Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid request header format. HTTP Version not found",
    ))?.to_owned();
    return Ok((method, path, version));
}

fn read_first_line(stream: &mut BufReader<TcpStream>) -> Result<String, std::io::Error> {
    let mut buffer = Vec::with_capacity(4096);
    while let Some(Ok(byte)) = stream.bytes().next() {
        if byte.eq(&b'\n') {
            if buffer.ends_with(b"\r") {
                buffer.pop();
            }
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
