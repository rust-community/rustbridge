extern crate hyper;

use hyper::Client;

fn main() {
    let client = Client::new();
    let response =
        client.get("https://brson.github.io/demo/wishlist.html")
        .send()
        .expect("Request failed");
    println!("{:?}", response);
}
