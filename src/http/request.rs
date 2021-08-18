use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug};
use std::fmt::Result as fmtresult;
use std::fmt::Formatter;
use std::str;
use std::str::Utf8Error;
use super::method::MethodError;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequestError)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequestError)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequestError)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocolError);
        }

        let method: Method= method.parse()?;

        let mut query_string = None;
        path.find('?')

        unimplemented!();
    }


}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            Some((&request[..i],&request[i+1..]));
        }
    }

    None
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncodingError
    }
}

impl From<MethodError> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidMethodError
    }
}

pub enum ParseError {
    InvalidRequestError,
    InvalidEncodingError,
    InvalidProtocolError,
    InvalidMethodError
}



impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> fmtresult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> fmtresult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequestError => "Invalud Request",
            Self::InvalidEncodingError => "Invalud Encoding",
            Self::InvalidProtocolError => "Invalud Protocol",
            Self::InvalidMethodError => "Invalud Method",

        }
    }
}


impl Error for ParseError {

}