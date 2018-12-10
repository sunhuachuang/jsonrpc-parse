use bytes::Bytes;
use serde_json::Value;

use crate::parse::split_bytes;
use crate::response::Error;
use crate::types::Params;

#[derive(Default, Debug, Clone)]
pub struct Request {
    jsonrpc: String,
    method: String,
    id: String,
    params: Params,
}

impl Request {
    pub fn new(method: String, id: String, params: Params) -> Self {
        let jsonrpc = "2.0".into();

        Request {
            jsonrpc,
            method,
            id,
            params,
        }
    }

    pub fn parse_from_json(value: Value) -> Result<Self, Error> {
        let id = if let Some(id) = value.get("id") {
            id.as_str().unwrap().into()
        } else {
            " ".into()
        };

        // check if json is response
        if value.get("result").is_some() || value.get("error").is_some() {
            return Err(Error::InvalidRequest("".into(), id));
        }

        if value.get("method").is_none() {
            return Err(Error::MethodNotFound("".into(), id));
        }

        let method = value.get("method").unwrap().as_str().unwrap().into();

        let params = if let Some(params) = value.get("params") {
            params.clone()
        } else {
            Value::Null
        };

        let jsonrpc = "2.0".into();

        Ok(Request {
            jsonrpc,
            method,
            id,
            params,
        })
    }

    pub fn parse(bytes: Bytes) -> Result<Self, Error> {
        split_bytes(bytes).and_then(|value| Request::parse_from_json(value))
    }

    pub fn deparse(&self) -> Bytes {
        Default::default()
    }

    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn params(&self) -> &Params {
        &self.params
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}
