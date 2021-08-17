use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug};
use std::fmt::Result as fmtresult;
use std::fmt::Formatter;
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        unimplemented!();
    }


}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncodingError
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