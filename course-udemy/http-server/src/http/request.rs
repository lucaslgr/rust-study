use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}

impl TryFrom<&[u8]> for Request {
  type Error = ParseError;

  fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
  //   match str::from_utf8(buf) {
  //     Ok(request) => {},
  //     Err(_) => return Err(ParseError::InvalidEncoding),
  //   }

  //   match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
  //     Ok(request) => {},
  //     Err(e) => return Err(e)
  //   }

  // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

    //After to implement the From trait to ParseError we can do this straight way below:
    let request = str::from_utf8(buf)?;

    match get_next_word(request) {
      Some((method, request)) => {},
      None => return Err(ParseError::InvalidRequest),
    }

    let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    let method: Method= method.parse()?;

    let mut query_string = None;
    
    if let Some(i) = path.find('?') {
      query_string = Some(&path[i+1..]);
      path = &path[..i];
    }


    unimplemented!()
  }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
  for (i, val) in request.chars().enumerate() {
    if val == ' ' || val == '\r' || val == '\n' {
      return Some((&request[..i], &request[i+1..]));
    }
  }
  None
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethodError,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethodError => "Invalid MethodError",
    }
  }
}

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethodError
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Error for ParseError {}
