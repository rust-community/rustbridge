#![allow(non_snake_case)]
extern crate hyper;
extern crate zip;
extern crate rustlearn;

use std::io::{Cursor, Read};

use hyper::client::Client;
use zip::read::ZipArchive;
use rustlearn::prelude::*;
use rustlearn::feature_extraction::DictVectorizer;
use rustlearn::cross_validation::CrossValidation;
use rustlearn::linear_models::sgdclassifier;
use rustlearn::metrics::accuracy_score;


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

        // Convert the labels to binary
        labels.push(if label == "Spam" {
            0.0
        } else {
            1.0
        });

        // The vectorizer will keep a mapping from tokens
        // to column indices.
        for token in text.split_whitespace() {
            vectorizer.partial_fit(row_num, token, 1.0);
        }
    }

    (vectorizer.transform(), Array::from(labels))
}


fn fit(X: &SparseRowArray, y: &Array) -> f32 {

    let num_epochs = 10;
    let num_folds = 10;

    let mut accuracy = 0.0;

    for (train_indices, test_indices) in CrossValidation::new(y.rows(), num_folds) {

        let X_train = X.get_rows(&train_indices);
        let X_test = X.get_rows(&test_indices);

        let y_train = y.get_rows(&train_indices);
        let y_test = y.get_rows(&test_indices);

        let mut model = sgdclassifier::Hyperparameters::new(X.cols())
            .learning_rate(0.05)
            .l2_penalty(0.01)
            .build();

        for _ in 0..num_epochs {
            model.fit(&X_train, &y_train);
        }

        let fold_accuracy = accuracy_score(&y_test, &model.predict(&X_test).unwrap());

        accuracy += fold_accuracy;
    }

    accuracy / num_folds as f32
}


fn main() {
    let zipped = download("https://archive.ics.uci.\
                           edu/ml/machine-learning-databases/00228/smsspamcollection.zip");
    let raw_data = unzip(zipped);

    for line in raw_data.lines().take(3) {
        println!("{}", line);
    }
    
    let (X, y) = parse(&raw_data);

    println!("X: {} rows, {} columns, {} non-zero entries.",
             X.rows(), X.cols(), X.nnz());
    
    println!("Accuracy: {}", fit(&X, &y));

}
