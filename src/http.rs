use std::{fmt::{Display}};

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

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::OnePointOne => "1.1",
        };
        f.write_str(s)
    }
}


#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug)]
pub struct HeaderParseError;

impl TryFrom<&str> for Header {
    type Error = HeaderParseError;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut split = line.split(": ");
        let name = split.next().ok_or(HeaderParseError)?.to_string();
        let value = split.next().ok_or(HeaderParseError)?.to_string();

        Ok(Self { name, value })
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

#[derive(Debug)]
pub struct Message {
    pub version: Version,
    pub headers: Vec<Header>,
    pub body: Option<Body>,
}

impl Message {
    pub fn new(version: Version, headers: Vec<Header>, body: Option<Body>) -> Self {
        Self {
            version,
            headers,
            body,
        }
    }
}

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

fn parse_body(_lines: &mut std::iter::Peekable<std::str::Lines>) -> Option<Body> {
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
    pub status_code: StatusCode,
    pub message: Message,
}

impl Response {
    pub fn serialize(&self) -> Vec<u8> {
        let version_string = self.message.version.to_string();
        let status_code = StatusCode::OK as u16;
        let status_code_string = self.status_code.to_string();
        let first_line = format!("HTTP/{version_string} {status_code} {status_code_string}");

        let header_lines: Vec<String> = self.message.headers.iter().map(Header::to_string).collect();
        let headers_string = header_lines.join("\n");

        let body_lines = match &self.message.body {
            Some(body) => body.to_string(),
            None => String::new()
        };

        let response_str = format!("{first_line}\n{headers_string}\n{body_lines}");

        response_str.as_bytes().to_vec()

    }
}
