use std::string;
use std::fmt;
use std::error::Error;

use rustc_serialize::json;


#[derive(Debug)]
pub enum DeserializeError {
    Utf8(string::FromUtf8Error),
    Decode(json::DecoderError),
}

impl From<string::FromUtf8Error> for DeserializeError {
    fn from(err: string::FromUtf8Error) -> DeserializeError {
        DeserializeError::Utf8(err)
    }
}

impl From<json::DecoderError> for DeserializeError {
    fn from(err: json::DecoderError) -> DeserializeError {
        DeserializeError::Decode(err)
    }
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializeError::Utf8(ref err) => err.fmt(f),
            DeserializeError::Decode(ref err) => err.fmt(f),
        }
    }
}

impl Error for DeserializeError {
    fn description(&self) -> &str {
        match *self {
            DeserializeError::Utf8(ref err) => err.description(),
            DeserializeError::Decode(ref err) => err.description(),
        }
    }
}
