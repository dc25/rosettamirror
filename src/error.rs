use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RosettaError {
    UnexpectedFormat,
    UnrecognizedLanguage (String)
}

struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl fmt::Display for RosettaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RosettaError::UnexpectedFormat => 
                write!(f, "Unknown format."),

            RosettaError::UnrecognizedLanguage(l) => 
				write!(f, "There is an error: {}", l),

        }
    }
}

impl Error for RosettaError { }
