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
                             .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    let document = Document::from(body.as_str());
    let rows = document.find(Class("a-row"));
    for row in rows.iter() {
        let maybe_title_node = row.find(Name("h5")).first();
        let maybe_price_node = row.find(Class("a-color-price")).first();
        if let (Some(title_node), Some(price_node)) = (maybe_title_node, maybe_price_node) {
            let title = title_node.text().trim().to_string();
            let price = price_node.text().trim().to_string();
            println!(" * Book \"{}\", with price {}", title, price);
        }
    }
}
