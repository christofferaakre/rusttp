use log::debug;

#[derive(Debug)]
pub enum Method {
    HEAD,
    GET,
    POST,
}

#[derive(Debug)]
pub enum Version {
    OnePointOne,
}

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct HeaderParseError;

impl TryFrom<&str> for Header {
    type Error = HeaderParseError;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut split = line.split(": ");
        let name = split.next().ok_or(HeaderParseError)?.to_string();
        let value = split.next().ok_or(HeaderParseError)?.to_string();

        Ok(Self {
            name, value
        })

    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum StatusCode {
    OK = 200,
    Error(String) = 400,
}

#[derive(Debug)]
struct Body {
    contents: Vec<u8>,
}

#[derive(Debug)]
struct Message {
    version: Version,
    headers: Vec<Header>,
    body: Option<Body>,
}

#[derive(Debug)]
pub struct Request {
    method: Method,
    target: String,
    message: Message,
}

#[derive(Debug)]
pub enum HttpRequestError {
    ParseError,
    UnsupportedHttpVersion
}

impl TryFrom<&str> for Request {
    type Error = HttpRequestError;
    fn try_from(input: &str) -> Result<Self,  Self::Error> {
        // Example request string: 
        // GET / HTTP/1.1
        // Host: 127.0.0.1:8080
        // User-Agent: curl/7.87.0
        // Accept: */*
        let mut lines = input.lines().peekable();
        let mut split = lines.next().unwrap().split_whitespace();
        let method = match split.next().unwrap() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => {
                return Err(Self::Error::ParseError);
            }

        };

        // skip the " / " part of the request
        split.next().unwrap();

        let version = match split.next().unwrap() {
            "HTTP/1.1" => Version::OnePointOne,
            _ => {
                return Err(Self::Error::UnsupportedHttpVersion)
            }
        };

        let target = lines.next().unwrap().split_whitespace().nth(1).unwrap().to_string();

        let headers = parse_headers(&mut lines);
        let body = parse_body(&mut lines);

        let message = Message {
            version,
            headers, 
            body
        };

        Ok(Self {
            target, message, method
        })


    }
}

fn parse_body(lines: &mut std::iter::Peekable<std::str::Lines>) -> Option<Body> {
    None
}

fn parse_headers(lines: &mut std::iter::Peekable<std::str::Lines>) -> Vec<Header> {
    let mut headers = Vec::new();
    loop {
        let line = lines.peek().unwrap();
        let header = Header::try_from(*line);
        match header {
            Ok(header) => {
                headers.push(header);
                lines.next().unwrap();
            }
            Err(err) => {
                debug!("Failed to parse header from line {line}: {err:?}");
                return headers;
            }
        }
    }
}


pub struct Response {
    status_code: StatusCode,
    message: Message,
}
