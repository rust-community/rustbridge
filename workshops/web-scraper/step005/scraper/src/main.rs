extern crate hyper;

use hyper::Client;

fn main() {
    let client = Client::new();
    let mut response =
        client.get("https://brson.github.io/demo/wishlist.html")
        .send()
        .unwrap();
    println!("{:?}", response);
}
