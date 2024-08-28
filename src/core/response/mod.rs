use http::StatusCode;
use httpdate::fmt_http_date;
use std::io::Write;
use std::net::TcpStream;
use std::time::SystemTime;

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
            Date: {}
            Server: {}
            Content-Length: {}
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
impl IntoResponse for String {
    fn build_response_body(self) -> ResponseBody {
        return ResponseBody { content: self };
    }
}
