#![allow(non_snake_case)]
extern crate hyper;
extern crate zip;
extern crate rustlearn;

use std::io::{Cursor, Read};

use hyper::client::Client;
use zip::read::ZipArchive;
use rustlearn::prelude::*;
use rustlearn::feature_extraction::DictVectorizer;


fn download(url: &str) -> Vec<u8> {

    let client = Client::new();
    let mut response = client.get(url).send().unwrap();

    let mut data = Vec::new();
    response.read_to_end(&mut data).unwrap();

    data
}


fn unzip(zipped: Vec<u8>) -> String {
    let mut archive = ZipArchive::new(Cursor::new(zipped)).unwrap();
    let mut file = archive.by_name("SMSSpamCollection").unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}


fn parse(data: &str) -> (SparseRowArray, Array) {

    // Initialise the vectorizer
    let mut vectorizer = DictVectorizer::new();
    let mut labels = Vec::new();

    // Like Python enumerate(), this will iterate
    // over pairs of (row_number, line).
    for (row_num, line) in data.lines().enumerate() {
        // The label and text message content is separated by a tab.
        // We split the line in two here.
        let (label, text) = line.split_at(line.find('\t').unwrap());

        // Convert the labels to binary. We use pattern matching
        // to ensure that the program is aborted if an unexpected
        // label is encountered.
        labels.push(match label {
            "spam" => 0.0,
            "ham" => 1.0,
            _ => panic!(format!("Invalid label: {}", label))
        });

        // The vectorizer will keep a mapping from tokens
        // to column indices.
        for token in text.split_whitespace() {
            vectorizer.partial_fit(row_num, token, 1.0);
        }
    }

    (vectorizer.transform(), Array::from(labels))
}


fn main() {
    let zipped = download("https://archive.ics.uci.\
                           edu/ml/machine-learning-databases/00228/smsspamcollection.zip");
    println!("Downloaded {} bytes of data", zipped.len());

    let raw_data = unzip(zipped);

    for line in raw_data.lines().take(3) {
        println!("{}", line);
    }
    
    let (X, y) = parse(&raw_data);

    println!("X: {} rows, {} columns, {} non-zero entries. Y: {:.2}% positive class",
             X.rows(), X.cols(), X.nnz(), y.mean() * 100.0);
}
