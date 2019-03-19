use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RosettaError {
    UnexpectedFormat,
    UnrecognizedLanguage (String)
}

impl fmt::Display for RosettaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RosettaError::UnexpectedFormat => 
                write!(f, "Unknown format."),

            RosettaError::UnrecognizedLanguage(l) => 
				write!(f, "Unrecognized language: {}", l),

        }
    }
}

impl Error for RosettaError { }
