extern crate hyper;

use std::io::Read;

use hyper::client::Client;


fn download(url: &str) -> Vec<u8> {

    let client = Client::new();
    let mut response = client.get(url).send().unwrap();

    let mut data = Vec::new();
    response.read_to_end(&mut data).unwrap();

    data
}


fn main() {
    let zipped = download("https://archive.ics.uci.\
                           edu/ml/machine-learning-databases/00228/smsspamcollection.zip");
    println!("Downloaded {} bytes of data", zipped.len());
}
