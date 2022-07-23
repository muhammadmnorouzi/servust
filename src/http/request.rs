use super::method::{Method, MethodError};
use std::str;
use std::str::Utf8Error;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Debug, Display, Result as FmtResult},
};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

pub enum ParsingError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// impl Debug for ParsingError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
//         Ok(self.message())
//     }
// }

impl From<MethodError> for ParsingError {
    fn from(_: MethodError) -> Self {
        ParsingError::InvalidMethod
    }
}

impl From<Utf8Error> for ParsingError {
    fn from(_: Utf8Error) -> Self {
        ParsingError::InvalidEncoding
    }
}

impl ParsingError {
    fn message(&self) -> &str {
        match self {
            ParsingError::InvalidRequest => "Invalid Request",
            ParsingError::InvalidEncoding => "Invalid Encoding",
            ParsingError::InvalidProtocol => "Invalid Protocol",
            ParsingError::InvalidMethod => "Invalid Method",
        }
    }
}

// impl Error for ParsingError {}

impl TryFrom<&[u8]> for Request {
    type Error = ParsingError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;

        // GET /things?for=atfp
        let (method, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParsingError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        todo!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}
