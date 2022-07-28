use super::method::{Method, MethodError};
use super::QueryString;
use std::fmt::Debug;
use std::str;
use std::str::Utf8Error;
use std::{
    convert::TryFrom,
    fmt::{Display, Result as FmtResult},
};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
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

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParsingError;

    fn try_from(buffer: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            // if !protocol.contains("HTTP/1.1") {
            return Err(ParsingError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string: Option<QueryString<'buf>> = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        return Ok(Self {
            path,
            query_string,
            method,
        });
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
