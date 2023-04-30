use std::fmt::Display;

mod request;
mod response;
mod message;
mod header;

pub use request::Request;
pub use response::Response;
pub use message::Message;
pub use header::Header;
pub use header::parse_headers;


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

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::OnePointOne => "1.1",
        };
        f.write_str(s)
    }
}

#[derive(Debug)]
#[repr(u16)]
pub enum StatusCode {
    OK = 200,
    // Error(String) = 400,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::OK => "OK",
            // Self::Error(_string) => "Error",
        };
        f.write_str(s)
    }
}


#[derive(Debug)]
pub struct Body {
    pub contents: Vec<u8>,
}

#[derive(Debug)]
pub struct BodyParseError;

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(self.contents.clone());
        match s {
            Ok(s) => f.write_str(s.as_str()),
            Err(_err) => write!(f, "Non-UTF8 bytes: {:?}", self.contents)
        }
    }
}



pub fn parse_body(_lines: &mut std::iter::Peekable<std::str::Lines>) -> Option<Body> {
    None
}



