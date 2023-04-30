use crate::http::Body;
use crate::http::Header;
use crate::http::Version;

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
