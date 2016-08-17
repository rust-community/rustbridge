# Spam Or Ham?

In this tutorial we'll try to carry out a simple classification task using Rust, starting with setting up a new project, through collecting and parsing data, to fitting the model and evaluation.

We'll proceed through the following steps:

1. Setting up a new Rust project
2. Programmatically downloading the data
3. Unpacking and parsing the data
4. Fitting a model and evaluating it

## Setting up a new project
We can set up a new project using Cargo, the Rust package manager. We can do that by executing
```
cargo new spam_or_ham --bin
```

This does a couple of things:

1. Creates a new directory, `spam_or_ham` that contains the structure of a typical Rust project.
2. It initializes a new Git repository in that directory.
3. It creates a file called `Cargo.toml` which contains the metadata describing our new package (called a `crate` in Rust) as well as the specification of it dependencies.
4. Finally, it creates a `main.rs` file in the `src/` subdirectory, which contains the entry point of our program. Because we passed the `--bin` flag to Cargo when creating the project, our project builds an executable binary.

At the beginning, our `main.rs` looks like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

We can run it to verify that it works through invoking Cargo:
```
> cargo run
>>>     Compiling spam_or_ham v0.1.0 (file:///home/maciej/Code/rustbridge/workshop/machine_learning/spam_or_ham)
>>>     Running `target/debug/spam_or_ham`
>>>     Hello, world!
```

`cargo run` builds the project (resolving any dependencies) and runs it; `cargo build` will simply compile it, and `cargo test` will run the tests (but we don't have any yet).

## Getting the data
For this example, we'll use the [SMS Spam Dataset](https://archive.ics.uci.edu/ml/datasets/SMS+Spam+Collection#). It contains 5574 text messages labelled as either spam or ham. The data looks roughly like this:

```
ham	Go until jurong point, crazy.. Available only in bugis n great world la e buffet... 
ham	Ok lar... Joking wif u oni...
spam	Free entry in 2 a wkly comp to win FA Cup final tkts 21st May 2005.
ham	U dun say so early hor... U c already then say...
```

### Downloading the data
Let's write a function to download the data.

To do this, we need to talk about dependencies: the Rust standard library has a fairly narrow focus, and a lot of key tasks are delegated to third-party libraries (called crates). For downloading the dataset we'll use a package called `hyper`.

The package repository is [crates.io](crates.io). The crates.io `hyper` [page](https://crates.io/crates/hyper) gives us a couple of pieces of information about the package

1. A link to its [documentation](http://hyperium.github.io/hyper)
2. A link to its [repository](https://github.com/hyperium/hyper)
3. Its download statistics. This is a good guide to figuring out which library is the standard way of doing a given thing in Rust.
4. Its `Cargo.toml` line: the line you to include in your `Cargo.toml` file to include it as a dependency for your project.

We copy that line into the dependencies section of `Cargo.toml`. It should look like this:
```
[package]
name = "spam_or_ham"
version = "0.1.0"

[dependencies]
hyper = "0.9.10"
```

That's it! If we run `cargo run`, we should see output similar to the following:
```
> cargo run
>>> Updating registry `https://github.com/rust-lang/crates.io-index`
>>> Downloading hyper v0.9.10
>>> Downloading language-tags v0.2.2
>>> Downloading mime v0.2.2
>>> <snip>
>>> Compiling cookie v0.2.5
>>> Compiling hyper v0.9.10
>>> Compiling spam_or_ham v0.1.0
>>> Running `target/debug/spam_or_ham`
>>> Hello, world!
```

This means cargo has downloaded and compiled `hyper` (and all of its dependencies) before building our project. We are now set to start using it!

Our download function will look something like this

```rust
fn download(url: &str) -> Vec<u8> {
   // snip
}
```

We download a resource by its URL and return an array of bytes (a `Vec<u8>>`). In order to start writing the body of the function, we need to impor the `hyper` dependency. We do this by putting `extern crate hyper` at the top of `main.rs`. This imports the `hyper` module into the scope of our project.

Looking at the GET example in the `hyper` [documentation](http://hyper.rs/hyper/v0.9.10/hyper/client/index.html#get), we should be able to write somthing along the lines of
```rust
let client = Client::new();
let mut response = client.get(url).send().unwrap();
```

The first line instantiates a HTTP client; the second one makes the request. There are two interesting things about it.

Firstly, the [return type](http://hyper.rs/hyper/v0.9.10/hyper/client/struct.RequestBuilder.html#method.send) of `send()` is `Result<Response>` --- why is that? Rust uses `Result` types to handle the results of computations that can fail. In this instance, downloading the data can succeed (in which case we would get an `Ok<Response>` variant of `Result<Response>`), or fail (due to network issues, a moved resource and so on). In that case, we'd get an `Err` variant. That the compiler then forces us to properly handle both cases is part of Rust's focus on safety.

So how do we handle a `Result` type? Rust provides an extremely powerful [pattern matching](https://doc.rust-lang.org/book/patterns.html) paradigm, but in this simple case we're just going to skip error handling and call `unwrap` on all of the `Result`s we encounter. This causes the program to abort whenever there is an error.

Secondly, we want to bind the resulting response to a mutable variable, and so we use the `mut` modifier when binding the response variable. While Rust's [mutability](https://doc.rust-lang.org/book/mutability.html) system is simple, it is somewhat beyond the scope of this tutorial. All we need to know is that we modify the response when reading from it, so we need to declare it as mutable.


Once we have our response we want to convert it to a byte array with something like the following:

```rust
let mut data = Vec::new();
response.read_to_end(&mut data).unwrap()
```

For this to work, we also need to import the `Read` [trait](https://doc.rust-lang.org/book/traits.html) into our scope by adding `use std::io::Read;` at the top of the file. The reasons for this are somewhat arcane so we'll skip them here.

Once that's completed we simply return the `data` variable by including it on the last line of the function (the last line of any expression is its return value):
```rust
fn download(url: &str) -> Vec<u8> {

    let client = Client::new();
    let mut response = client.get(url).send().unwrap();

    let mut data = Vec::new();
    response.read_to_end(&mut data).unwrap();

    data
}
```

### Unzipping the data
We have downloaded a zipped archive: the next step is to unzip it. We'll need another dependency to do that, the [zip](https://crates.io/crates/zip) crate. As before, we add it to the `Cargo.toml` file

```
[dependencies]
hyper = "0.9.10"
zip = "0.1.18"
```

and then import it in `main.rs`, together with the `ZipArchive` struct:

```rust
extern crate zip;
<snip>
use zip::read::ZipArchive;
```

Our unzip function will take the vector of bytes from the download function, and return a `String`. It could look like this:

```rust
// We need to add the Cursor import 
use std::io::{Cursor, Read};

<snip>

fn unzip(zipped: Vec<u8>) -> String {
    let mut archive = ZipArchive::new(Cursor::new(zipped)).unwrap();
    let mut file = archive.by_name("SMSSpamCollection").unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}
```

We open the archive, select the file we want from it, and read it into a `String` which we then return (dealing with `Result` types by unwrapping them.

We can print some lines as a quick sanity check:

```rust
fn main() {
    let zipped = download("https://archive.ics.uci.\
                           edu/ml/machine-learning-databases/00228/smsspamcollection.zip");
    let raw_data = unzip(zipped);

    for line in raw_data.lines().take(3) {
        println!("{}", line);
    }
}
```

This gives us a first look at Rust iterators `raw_data.lines()` creates an iterator over slices of the string, separates by newlines; we then take 3 elements from it and print them. Rust's functional features manifest themselves in a a large array of powerful [iterator adapters](https://doc.rust-lang.org/book/iterators.html#iterator-adaptors) which are not only convenient but also compile to efficient machine code equivalent to traditional C and C++ for loops.

### Building training matrices
We're going to use a package called [rustlearn](https://crates.io/crates/rustlearn) for model fitting an evaluation. As before, we add it to `Cargo.toml` and import it:
```
[dependencies]
hyper = "0.9.10"
zip = "0.1.18"
rustlearn = "0.4.0"
```

```rust
extern crate rustlearn;

<snip>

use rustlearn::prelude::*;
```

The first step is to transform the data into a feature matrix and a target array. We'll transform every label into either a 1 (ham) or 0 (spam), and use one-hot-encoded bag of words features. For one-hot-encoding we're going to use [DictVectorizer](https://maciejkula.github.io/rustlearn/doc/rustlearn/feature_extraction/dict_vectorizer/struct.DictVectorizer.html), and return a sparse array for features and a dense array for labels:

```rust
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
```

Calling the function

```rust
let (X, y) = parse(&raw_data);

println!("X: {} rows, {} columns, {} non-zero entries",
          X.rows(), X.cols(), X.nnz());
```

should print `X: 5574 rows, 15733 columns, 81085 non-zero entries`.


### Fitting and evaluating the model
