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
    let wrapper = document.find(Attr("id", "item-page-wrapper"));
    let rows = wrapper.find(Class("a-fixed-right-grid"));

    for row in rows.iter() {
        let title_node = row.find(Name("h5")).first();
        let price_node = row.find(Class("a-color-price")).first();
        match (title_node, price_node) {
            (Some(title), Some(price)) => {
                println!("* Book \"{}\", with price {}",
                    title.text().trim(),
                    price.text().trim()
                );
            },
            (_, _) => (),
        }
    }
}
