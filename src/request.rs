use bytes::Bytes;
use std::collections::HashMap;

use crate::response::Error;

#[derive(Default, Debug, Clone)]
pub struct Request {
    http_header: HashMap<String, String>,
    method: String,
    id: String,
    params: (),
}

impl Request {
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
