use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RosettaError {
    UnexpectedFormat,
}
 

impl fmt::Display for RosettaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RosettaError::UnexpectedFormat => write!(f, "No matching cities with a \
                                             population were found."),
        }
    }
}

impl Error for RosettaError {
    fn description(&self) -> &str {
        match *self {
            RosettaError::UnexpectedFormat => "unknown format",
        }
    }
}
