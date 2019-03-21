use bytes::{BufMut, Bytes, BytesMut};
use tokio_codec::{Decoder, Encoder};

use super::request::Request;
use super::response::{Error, Response};

#[derive(Debug, Clone)]
pub enum HTTP {
    Request(Request),
    Response(Response),
    Error(Error),
    NeedMore(usize, usize, Bytes),
}

impl HTTP {
    fn parse(src: &mut BytesMut, mut had_body: BytesMut) -> Option<HTTP> {
        if had_body.len() == 0 {
            let (version, amt, length) = {
                let mut req_parsed_headers = [httparse::EMPTY_HEADER; 16];
                let mut res_parsed_headers = [httparse::EMPTY_HEADER; 16];
                let mut req = httparse::Request::new(&mut req_parsed_headers);
                let mut res = httparse::Response::new(&mut res_parsed_headers);
                let req_status = req.parse(&src);
                let res_status = res.parse(&src);

                if req_status.is_err() && res_status.is_err() {
                    println!("failed to parse http");
                    return Some(HTTP::Error(Error::ParseError(None)));
                }

                let (status, version, length) = if req_status.is_err() {
                    let content_length_headers: Vec<httparse::Header> = res
                        .headers
                        .iter()
                        .filter(|header| header.name == "Content-Length")
                        .cloned()
                        .collect();

                    if content_length_headers.len() != 1 {
                        return Some(HTTP::Error(Error::ParseError(None)));
                    }

                    let length_bytes = content_length_headers.first().unwrap().value;
                    let mut length_string = String::new();

                    for b in length_bytes {
                        length_string.push(*b as char);
                    }

                    let length = length_string.parse::<usize>();
                    if length.is_err() {
                        return Some(HTTP::Error(Error::ParseError(None)));
                    };

                    (res_status.unwrap(), res.version.unwrap(), length.unwrap())
                } else {
                    let content_length_headers: Vec<httparse::Header> = req
                        .headers
                        .iter()
                        .filter(|header| header.name == "Content-Length")
                        .cloned()
                        .collect();

                    if content_length_headers.len() != 1 {
                        return Some(HTTP::Error(Error::ParseError(None)));
                    }

                    let length_bytes = content_length_headers.first().unwrap().value;
                    let mut length_string = String::new();

                    for b in length_bytes {
                        length_string.push(*b as char);
                    }

                    let length = length_string.parse::<usize>();
                    if length.is_err() {
                        return Some(HTTP::Error(Error::ParseError(None)));
                    };

                    (req_status.unwrap(), req.version.unwrap(), length.unwrap())
                };

                let amt = match status {
                    httparse::Status::Complete(amt) => amt,
                    httparse::Status::Partial => return Some(HTTP::Error(Error::ParseError(None))),
                };

                (version, amt, length)
            };
            if version != 1 {
                println!("only HTTP/1.1 accepted");
                return Some(HTTP::Error(Error::ParseError(None)));
            }

            had_body = src.split_off(amt);

            if had_body.len() < length {
                return Some(HTTP::NeedMore(amt, length, had_body.freeze()));
            }
        }

        let json = had_body.freeze();

        let request_result = Request::parse_from_json_bytes(json.clone());
        if request_result.is_err() {
            let response_result = Response::parse_from_json_bytes(json);
            if response_result.is_err() {
                Some(HTTP::Error(request_result.err().unwrap()))
            } else {
                Some(HTTP::Response(response_result.unwrap()))
            }
        } else {
            Some(HTTP::Request(request_result.unwrap()))
        }
    }

    fn deparse(&self) -> Bytes {
        match self {
            HTTP::Request(meta) => meta.deparse(),
            HTTP::Response(meta) => meta.deparse(),
            HTTP::Error(meta) => meta.deparse(),
            _ => Bytes::new(),
        }
    }
}

// cache, body length, header_length, is_receiving
#[derive(Default)]
pub struct HTTPCodec(pub BytesMut, pub usize, pub usize, pub bool);

impl HTTPCodec {
    pub fn new() -> Self {
        HTTPCodec(BytesMut::new(), 0, 0, true)
    }
}

impl Decoder for HTTPCodec {
    type Item = HTTP;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if !self.3 {
            return Ok(None);
        }
        if self.2 > 0 {
            let bytes = src.split_off(self.2);
            self.0.reserve(bytes.len());
            self.0.put(bytes);

            if self.0.len() < self.1 {
                return Ok(None);
            }
        }

        let http = HTTP::parse(src, self.0.clone());
        match http {
            Some(HTTP::NeedMore(amt, length, bytes)) => {
                self.0.reserve(bytes.len());
                self.0.put(bytes);
                self.1 = length; // body leangth
                self.2 = amt; // header length
                Ok(None)
            }
            Some(h) => {
                self.3 = false;
                self.0.clear();
                self.0.reserve(0);
                Ok(Some(h))
            }
            None => Ok(None),
        }
    }
}

impl Encoder for HTTPCodec {
    type Item = HTTP;
    type Error = std::io::Error;

    fn encode(&mut self, msg: HTTP, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = msg.deparse();
        dst.reserve(bytes.len());
        dst.put(bytes);
        Ok(())
    }
}
