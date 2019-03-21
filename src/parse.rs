use crate::response::Error;
use bytes::{BufMut, Bytes, BytesMut};
use serde_json::Value;

pub fn split_bytes(bytes: Bytes) -> Result<Value, Error> {
    let mut vec: Vec<u8> = Vec::new();

    for (i, v) in (&bytes).iter().enumerate() {
        if v == &13 || v == &10 {
            vec.push(v.clone())
        } else {
            if vec == [13, 10, 13, 10] {
                let b = Bytes::from(&bytes[i..]);
                return serde_json::from_slice(&b[..]).or(Err(Error::ParseError(None)));
            } else {
                if vec.len() > 0 {
                    vec.clear();
                }
            }
        }
    }
    println!("!!!!!! error parse");

    return Err(Error::ParseError(None));
}

pub fn generate_request_headers(host: String, length: usize) -> BytesMut {
    let mut headers = BytesMut::with_capacity(length + 200);
    headers.put("POST / HTTP/1.1\r\n");
    headers.put("Host: ");
    headers.put(host);
    headers.put("\r\n");
    headers.put("Content-Type: application/json\r\n");
    headers.put("Content-Length: ");
    headers.put(length.to_string());
    headers.put("\r\n\r\n");
    headers
}

pub fn generate_response_headers(length: usize) -> BytesMut {
    let mut headers = BytesMut::with_capacity(length + 200);
    headers.put("HTTP/1.1 200 OK\r\n");
    headers.put("Server: Tachion JSON-RPC\r\n");
    headers.put("Content-Type: application/json\r\n");
    headers.put("Connection: Closed\r\n");
    headers.put("Content-Length: ");
    headers.put(length.to_string());
    headers.put("\r\n\r\n");
    headers
}
