use super::Message;
use super::StatusCode;
use super::Header;

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
