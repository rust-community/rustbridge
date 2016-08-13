extern crate byteorder;
extern crate rustc_serialize;
extern crate rusqlite;
extern crate time;
#[macro_use]
extern crate nickel;

mod serdes;
mod server;
mod db;

use db::{LinksDB};
use server::start_server;

fn main() {
    for i in 0..300 {
        serdes::encode(i);
        serdes::decode(serdes::encode(i));
    }

    // Example db usage
    let links_db = LinksDB::with_tables().unwrap();
    links_db.insert_link("example.com").unwrap();
    let link = links_db.link_for_url("example.com").unwrap();
    println!("Encoded ID: {}", serdes::encode(link.id as u64));

    start_server();
}
