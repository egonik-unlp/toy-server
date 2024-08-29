use http::StatusCode;
use httpdate::fmt_http_date;
use std::io::Write;
use std::net::TcpStream;
use std::time::SystemTime;

pub struct ResponseBody {
    content: String,
}
#[derive(Debug)]
pub struct Response {
    code: StatusCode,
    body: String,
    // headers: Hesaders,
}
impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {}\r\nDate: {}\r\nServer: {}\r\nContent-Length: {}\r\nContent-Type : {}\r\n\r\n{}",
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
        let response = Response {
            code: code,
            body: body,
            // headers: Headers(headers_for_now),
        };
        return response;
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
