pub enum Method {
    HEAD,
    GET,
    POST
}

pub enum Version {
    OnePointOne,
    Two,
}

pub struct Header {
    name: String,
    value: String,
}

#[repr(u16)]
pub enum StatusCode {
    OK = 200,
    Error(String) = 400,
}

struct Body {
    contents: Vec<u8>,
}

struct Message {
    version: Version,
    headers: Vec<Header>,
    body: Option<Body>,
}

pub struct Request {
    method: Method,
    target: String,
    message: Message
}

pub struct Response {
    status_code: StatusCode,
    message: Message,
}
