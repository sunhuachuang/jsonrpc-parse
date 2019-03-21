use bytes::{BufMut, Bytes};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::parse::generate_response_headers;
use crate::parse::split_bytes;
use crate::types::Params;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    jsonrpc: String,
    method: String,
    id: String,
    result: Params,
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

    pub fn parse(bytes: Bytes) -> Result<Self, Error> {
        split_bytes(bytes).and_then(|value| Response::parse_from_json(value))
    }

    pub fn parse_from_json_bytes(bytes: Bytes) -> Result<Self, Error> {
        serde_json::from_slice(&bytes[..])
            .or(Err(Error::ParseError(None)))
            .and_then(|value| Response::parse_from_json(value))
    }

    pub fn parse_from_json(value: Value) -> Result<Self, Error> {
        let id = if let Some(id) = value.get("id") {
            id.as_str().unwrap().into()
        } else {
            " ".into()
        };

        if value.get("method").is_none() {
            return Err(Error::MethodNotFound("".into(), id));
        }
        let method = value.get("method").unwrap().as_str().unwrap().into();

        if value.get("result").is_some() {
            let jsonrpc = "2.0".into();
            let result = value.get("result").unwrap().clone();

            return Ok(Response {
                jsonrpc,
                method,
                id,
                result,
            });
        }

        if value.get("error").is_some() {
            let code = value
                .get("error")
                .unwrap()
                .get("code")
                .map_or(-32600, |v| v.as_i64().map_or(-32600, |v| v));

            let message = value
                .get("error")
                .unwrap()
                .get("message")
                .map_or("Invalid Request", |v| {
                    v.as_str().map_or("Invalid Request", |v| v)
                })
                .into();
            return Err(Error::ErrorResponse(method, id, code, message));
        }

        return Err(Error::InvalidResponse(method, id));
    }

    pub fn deparse(&self) -> Bytes {
        let body = serde_json::to_string(&self).unwrap();

        let body_bytes = body.as_bytes();

        let mut headers = generate_response_headers(body_bytes.len());
        headers.put(body_bytes);
        headers.freeze()
    }

    pub fn result(&self) -> &Params {
        &self.result
    }

    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorValue {
    code: i64,
    message: String,
}

impl ErrorValue {
    fn new(code: i64, message: String) -> Self {
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
    InvalidResponse(String, String),
    ErrorResponse(String, String, i64, String),
}

impl Error {
    fn error_value(&self) -> ErrorValue {
        match self {
            Error::ParseError(_) => ErrorValue::new(-32700, "Parse error".into()),
            Error::MethodNotFound(_, _) => ErrorValue::new(-32601, "Method not found".into()),
            Error::InvalidRequest(_, _) => ErrorValue::new(-32600, "Invalid Request".into()),
            Error::InvalidResponse(_, _) => ErrorValue::new(-32600, "Invalid Response".into()),
            Error::ErrorResponse(_, _, code, message) => ErrorValue::new(*code, message.clone()),
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
                    serde_json::to_string(&ErrorOnlyResponse::new(self.error_value()))
                        .map_err(|e| {
                            println!("{:?}", e);
                        })
                        .unwrap()
                }
                Error::MethodNotFound(method, id)
                | Error::InvalidRequest(method, id)
                | Error::InvalidResponse(method, id) => serde_json::to_string(&ErrorResponse::new(
                    method.clone(),
                    id.clone(),
                    self.error_value(),
                ))
                .unwrap(),
                Error::ErrorResponse(method, id, _, _) => serde_json::to_string(
                    &ErrorResponse::new(method.clone(), id.clone(), self.error_value()),
                )
                .unwrap(),
            };

        let body_bytes = body.as_bytes();

        let mut headers = generate_response_headers(body_bytes.len());
        headers.put(body_bytes);
        headers.freeze()
    }
}
