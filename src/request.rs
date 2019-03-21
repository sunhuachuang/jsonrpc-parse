use bytes::{BufMut, Bytes};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::parse::{generate_request_headers, split_bytes};
use crate::response::Error;
use crate::types::Params;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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
            if id.is_number() {
                id.as_u64().unwrap_or(0).to_string()
            } else if id.is_string() {
                id.as_str().unwrap().into()
            } else {
                " ".into()
            }
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

    pub fn parse_from_json_bytes(bytes: Bytes) -> Result<Self, Error> {
        serde_json::from_slice(&bytes[..])
            .or(Err(Error::ParseError(None)))
            .and_then(|value| Request::parse_from_json(value))
    }

    pub fn deparse(&self) -> Bytes {
        let body = serde_json::to_string(&self).unwrap();

        let body_bytes = body.as_bytes();

        let mut headers =
            generate_request_headers("Hyperdrive_RPC_Request".into(), body_bytes.len());
        headers.put(body_bytes);
        headers.freeze()
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
