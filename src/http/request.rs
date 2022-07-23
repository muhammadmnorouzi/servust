use super::method::Method;
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
        todo!()
    }
}
