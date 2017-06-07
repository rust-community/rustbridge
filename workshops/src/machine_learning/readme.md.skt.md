```rust,skt-1
extern crate hyper;
use hyper::*;

fn foo() {{
    let client = Client::new();
    let mut response = client.get("").send().unwrap();
    {}
}}

fn main() {{}}
```