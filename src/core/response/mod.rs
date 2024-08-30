use http::StatusCode;
use httpdate::fmt_http_date;
use std::fmt::Debug;
use std::io::Write;
use std::net::TcpStream;
use std::time::SystemTime;

#[derive(Debug)]
pub struct ResponseBody {
    pub content: String,
    pub content_type: String, 
}
#[derive(Debug)]
pub struct Response {
    pub(crate) code: StatusCode,
    pub(crate) body: ResponseBody,
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
            self.body.content.len(),
            self.body.content_type,
            self.body.content
        )
    }
}

impl Response {
    pub fn respond(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        


        return write!(stream, "{}", self);
    }
    pub fn new(code: StatusCode, body: String, content_type: String) -> Self {
        // let mut headers_for_now = HashMap::new();
        // headers_for_now.insert(ResponseHeaderType::Server, "EduServer".into());
        let response = Response {
            code: code,
            body: ResponseBody { content: body, content_type: content_type},
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
