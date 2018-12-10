use bytes::{BufMut, Bytes, BytesMut};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::parse::generate_response_headers;
use crate::types::Params;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    pub method: String,
    pub id: String,
    pub result: Params,
}

impl Response {
    pub fn new(method: String, id: String, result: Params) -> Self {
        let jsonrpc = "2.0".into();

        Response {
            jsonrpc,
            method,
            id,
            result,
        }
    }

    pub fn parse(_bytes: Bytes) -> Result<Self, Error> {
        Ok(Default::default())
    }

    pub fn deparse(&self) -> Bytes {
        let body = serde_json::to_string(&self).unwrap();

        let body_bytes = body.as_bytes();

        let mut headers = generate_response_headers(body_bytes.len());
        headers.put(body_bytes);
        headers.freeze()
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorValue {
    code: i32,
    message: String,
}

impl ErrorValue {
    fn new(code: i32, message: String) -> Self {
        ErrorValue { code, message }
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorOnlyResponse {
    jsonrpc: String,
    error: ErrorValue,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    jsonrpc: String,
    method: String,
    id: String,
    error: ErrorValue,
}

impl ErrorOnlyResponse {
    fn new(error: ErrorValue) -> Self {
        let jsonrpc = "2.0".into();
        ErrorOnlyResponse { jsonrpc, error }
    }
}

impl ErrorResponse {
    fn new(method: String, id: String, error: ErrorValue) -> Self {
        let jsonrpc = "2.0".into();
        ErrorResponse {
            jsonrpc,
            method,
            id,
            error,
        }
    }
}

// (String>, String) => method, id
#[derive(Debug, Clone)]
pub enum Error {
    ParseError(Option<(String, String)>),
    MethodNotFound(String, String),
    InvalidRequest(String, String),
}

impl Error {
    fn error_value(&self) -> ErrorValue {
        match self {
            Error::ParseError(_) => ErrorValue::new(-32700, "Parse error".into()),
            Error::MethodNotFound(_, _) => ErrorValue::new(-32601, "Method not found".into()),
            Error::InvalidRequest(_, _) => ErrorValue::new(-32600, "Invalid Request".into()),
        }
    }

    pub fn deparse(&self) -> Bytes {
        let body =
            match self {
                Error::ParseError(Some((method, id))) => serde_json::to_string(
                    &ErrorResponse::new(method.clone(), id.clone(), self.error_value()),
                )
                .unwrap(),
                Error::ParseError(None) => {
                    serde_json::to_string(&ErrorOnlyResponse::new(self.error_value())).unwrap()
                }
                Error::MethodNotFound(method, id) | Error::InvalidRequest(method, id) => {
                    serde_json::to_string(&ErrorResponse::new(
                        method.clone(),
                        id.clone(),
                        self.error_value(),
                    ))
                    .unwrap()
                }
            };

        let body_bytes = body.as_bytes();

        let mut headers = generate_response_headers(body_bytes.len());
        headers.put(body_bytes);
        headers.freeze()
    }
}
