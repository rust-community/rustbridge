extern crate hyper;
extern crate select;

use hyper::Client;

use select::document::Document;
use select::predicate::{Class, Name};

use std::io::Read;

fn main() {
    let client = Client::new();
    let mut response = client.get("https://brson.github.io/demo/wishlist.html")
                             .send()
                             .expect("Request failed");
    let mut body = String::new();
    response.read_to_string(&mut body).expect("Read failed");

    let document = Document::from(body.as_str());
    let rows = document.find(Class("a-row"));

    for row in rows.iter() {
        let maybe_name_node = row.find(Name("h5")).first();
        let maybe_price_node = row.find(Class("a-color-price")).first();
        if let (Some(name_node), Some(price_node)) = (maybe_name_node, maybe_price_node) {
            println!(
                " * Book \"{}\", with price {}",
                name_node.text().trim(),
                price_node.text().trim(),
            );
        }
    }
}
