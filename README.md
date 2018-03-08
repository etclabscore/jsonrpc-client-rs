# jsonrpc-client-core

A crate for generating transport agnostic, auto serializing, strongly typed JSON-RPC 2.0
clients.

This crate mainly provides a macro, `jsonrpc_client`. The macro generates structs that can be
used for calling JSON-RPC 2.0 APIs. The macro lets you list methods on the struct with
arguments and a return type. The macro then generates a struct which will automatically
serialize the arguments, send the request and deserialize the response into the target type.

## Transports

The `jsonrpc-client-core` crate itself and the structs generated by the `jsonrpc_client` macro
are transport agnostic. They can use any type implementing the `Transport` trait.

The main (and so far only) transport implementation is the Hyper based HTTP implementation
in the [`jsonrpc-client-http`](../jsonrpc_client_http/index.html) crate.

## Example

```rust
#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

use jsonrpc_client_http::HttpTransport;

jsonrpc_client!(pub struct FizzBuzzClient {
    /// Returns the fizz-buzz string for the given number.
    pub fn fizz_buzz(&mut self, number: u64) -> RpcRequest<String>;
});

fn main() {
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport
        .handle("https://api.fizzbuzzexample.org/rpc/")
        .unwrap();
    let mut client = FizzBuzzClient::new(transport_handle);
    let result1 = client.fizz_buzz(3).call().unwrap();
    let result2 = client.fizz_buzz(4).call().unwrap();
    let result3 = client.fizz_buzz(5).call().unwrap();

    // Should print "fizz 4 buzz" if the server implemented the service correctly
    println!("{} {} {}", result1, result2, result3);
}
```


License: MIT/Apache-2.0
