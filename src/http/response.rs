use std::io::{Result as IoResult, Write};

use super::HttpStatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: HttpStatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: HttpStatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
