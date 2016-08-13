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

use nickel::{Nickel, HttpRouter};
use rusqlite::SqliteConnection;
use time::Timespec;
use server::start_server;

//
// WEB SERVER
//
fn main() {
    for i in 0..300 {
        serdes::encode(i);
        serdes::decode(serdes::encode(i));
    }

    let links_db = LinksDB::with_tables().unwrap();
    start_server();
}
