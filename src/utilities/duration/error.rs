use std::{
    error,
    fmt
};

#[derive(Debug)]
pub struct ParseToDurationError(String);

impl fmt::Display for ParseToDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for ParseToDurationError { }
