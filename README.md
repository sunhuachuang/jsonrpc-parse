# jsonrpc-parse

parse JSON-RPC (2.0) from TCP Bytes or deparse JSON-RPC to Codec and Bytes

```
[dependencies]
jsonrpc_parse = "0.1"
```

```Rust
use jsonrpc_parse::httpcodec::{HTTPCodec, HTTP};

// use HTTPCodec to parse frame bytes, the result is HTTPCodec's HTTP

match http {
    HTTP::Request(req) => {}
    HTTP::Response(resp) => {}
    _ => {} // other is Response about the Error, can return direct.
}

```
