extern crate hyper;

use hyper::Client;
use std::io::Read;

fn main() {
    let client = Client::new();
    let mut response =
        client.get("https://brson.github.io/demo/wishlist.html")
              .send()
              .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    println!("{:?}", body);
}
