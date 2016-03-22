use std::string;
use std::fmt;
use std::io;
use std::error::Error;

use rustc_serialize::json;


// deserialization

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


// loading

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Deserialize(DeserializeError),
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> LoadError {
        LoadError::Io(err)
    }
}

impl From<DeserializeError> for LoadError {
    fn from(err: DeserializeError) -> LoadError {
        LoadError::Deserialize(err)
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoadError::Io(ref err) => err.fmt(f),
            LoadError::Deserialize(ref err) => err.fmt(f),
        }
    }
}

impl Error for LoadError {
    fn description(&self) -> &str {
        match *self {
            LoadError::Io(ref err) => err.description(),
            LoadError::Deserialize(ref err) => err.description(),
        }
    }
}
