use bytes::Bytes;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Response {
    http_header: HashMap<String, String>,
    method: String,
    id: String,
    result: (),
}

impl Response {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(_bytes: Bytes) -> Result<Self, Error> {
        Ok(Default::default())
    }

    pub fn deparse(&self) -> Bytes {
        Default::default()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Error {
    http_header: HashMap<String, String>,
    method: String,
    id: String,
    code: String,
    message: String,
}
