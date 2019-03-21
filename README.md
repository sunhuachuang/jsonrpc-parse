# jsonrpc-parse

parse JSON-RPC (2.0) from TCP Bytes or deparse JSON-RPC to Bytes

```Rust
use jsonrpc::httpcodec::{HTTPCodec, HTTP};

// if msg is HTTPCodec's HTTP

match msg {
    HTTP::Request(req) => {}
    HTTP::Response(resp) => {}
    _ => {} // other is Response about the Error, can return direct.
}

```
