use super::Message;
use super::{parse_body, parse_headers};
use super::Method;
use super::Version;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub target: String,
    pub message: Message,
    pub url: String,
}

#[derive(Debug)]
pub enum HttpRequestError {
    ParseError,
    UnsupportedHttpVersion,
}

impl TryFrom<&str> for Request {
    type Error = HttpRequestError;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
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
        let url = split.next().unwrap().to_string();

        let version = match split.next().unwrap() {
            "HTTP/1.1" => Version::OnePointOne,
            _ => return Err(Self::Error::UnsupportedHttpVersion),
        };

        let target = lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_string();

        let headers = parse_headers(&mut lines);
        let body = parse_body(&mut lines);

        let message = Message {
            version,
            headers,
            body,
        };

        Ok(Self {
            target,
            message,
            method,
            url,
        })
    }
}
