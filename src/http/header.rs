use log::debug;
use std::fmt::Display;

pub mod headers {
    pub const CONTENT_TYPE: &str = "Content-Type";
    pub mod content_type {
        pub const TEXT_PLAIN: &str = "text/plain";
        pub const TEXT_HTML: &str = "text/html";
    }
}

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
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

pub fn parse_headers(lines: &mut std::iter::Peekable<std::str::Lines>) -> Vec<Header> {
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
