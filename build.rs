extern crate skeptic;
extern crate walkdir;

use walkdir::*;

fn main() {
    let paths = WalkDir::new(".").into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_str().unwrap().to_string())
        .filter(|p| p.ends_with(".md"))
        .collect::<Vec<_>>();

    skeptic::generate_doc_tests(&paths[..]);
}
