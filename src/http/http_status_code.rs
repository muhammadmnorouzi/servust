use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum HttpStatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl HttpStatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            HttpStatusCode::Ok => "Ok",
            HttpStatusCode::BadRequest => "Bad Request",
            HttpStatusCode::NotFound => "Not Found",
        }
    }
}

impl Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}
