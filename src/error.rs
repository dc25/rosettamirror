#[derive(Debug)]
pub enum Error {
    /// Something went wrong with the HTTP request to the API.
    Http(reqwest::Error),
 
    /// There was a problem parsing the API response into JSON.
    Io(std::io::Error),
 
    /// There was a problem parsing the API response into JSON.
    ParseUrl(url::ParseError),
 
    /// There was a problem parsing the API response into JSON.
    SerdeJson(serde_json::Error),
 
    /// There was a problem parsing the API response into JSON.
    Onig(onig::Error),

    /// There was a problem parsing the API response into JSON.
    Regex(regex::Error),

    /// Unexpected JSON format from response
    UnexpectedFormat,
}
 
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Http(error)
    }
}
 
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}
 
impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::ParseUrl(error)
    }
}
 
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error)
    }
}
 
impl From<onig::Error> for Error {
    fn from(error: onig::Error) -> Self {
        Error::Onig(error)
    }
}
 
impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::Regex(error)
    }
}
