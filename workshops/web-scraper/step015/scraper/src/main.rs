extern crate hyper;
extern crate select;

use hyper::Client;

use select::document::Document;
use select::predicate::{Class};

use std::io::Read;

fn main() {
    let client = Client::new();
    let mut response = client.get("https://brson.github.io/demo/wishlist.html")
                             .send()
                             .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let document = Document::from(body.as_str());
    let rows = document.find(Class("a-row"));
    for row in rows.iter() {
        println!(" * Row {}", row.text());
    }
}
