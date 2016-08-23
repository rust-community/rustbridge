use nickel::{Nickel, HttpRouter, QueryString};
use serdes::{decode, encode};
use db::{LinksDB, Link};
use nickel::status::StatusCode;
use std::error::Error;

pub fn start_server(links_db: &LinksDB) {
    let mut server = Nickel::new();
    server.get("/admin", middleware! { |request, mut response|
        let url = request.query().get("url").unwrap();
        println!("This is the URL received: /{:?}", url);
        match links_db.insert_link(url) {
            Ok(id) => {
                let short_path = encode(id as u64);
                response.set(StatusCode::Ok);
                format!("Encoded URL path is {}", short_path)
            },
            Err(e) => {
                response.set(StatusCode::InternalServerError);
                String::from(e.description())
            }
        }
    });

    server.get("/:short_path", middleware! { |request, mut response|
        let short_path = request.param("short_path").unwrap();
        println!("This is the path received: {:?}", short_path);
        let id = decode(String::from(short_path));
        match links_db.find_link(id as i64) {
            Ok(link) => {
                response.set(StatusCode::Ok);
                format!("Decoded URL is {}", link.target)
            },
            Err(e) => {
                response.set(StatusCode::InternalServerError);
                String::from(e.description())
            }
        }
    });

    server.listen("127.0.0.1:6767");
}
