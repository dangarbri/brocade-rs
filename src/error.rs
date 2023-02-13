use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ResponseError {
    message: String
}
impl ResponseError {
    pub fn new(message: &str) -> ResponseError {
        ResponseError { message: message.to_string() }
    }
}
impl Error for ResponseError {}
impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}