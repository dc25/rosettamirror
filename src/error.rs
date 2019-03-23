use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RosettaError {
    UnexpectedFormat
}

impl fmt::Display for RosettaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RosettaError::UnexpectedFormat => 
                write!(f, "Unknown format."),
        }
    }
}

impl Error for RosettaError { }
