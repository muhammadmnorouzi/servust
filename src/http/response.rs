pub struct Response {
    status_code: HttpStatusCode,
    body: Option<String>,
}

pub enum HttpStatusCode {}

impl Response {
    pub fn new(status_code: HttpStatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }
}
