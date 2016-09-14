# Rust HTML Parsing Workshop

Suppose you have an Amazon.com wishlist of products and you want to find and
print a list the prices of each item using a command line program.

## Let's get started!

In this workshop we'll learn to use an external HTTP client library (`hyper`)
and an HTML DOM parser (`select`). We will take that data and process it using
`Iterators` into a product name and price which we will print to the screen.

## Step 000: Setup your local project

Open a command line terminal. This will be your primary way of building your
Rust code. Open a directory where you want to save your project and run.

```sh
cargo new --bin scraper
cd scraper
```

This command uses `cargo`, is the Rust Package Manager, to create a new project
structure that will build and run out of the box. Cargo simplifies the build and 
run steps of projects. The basic initial structure of a new cargo project has 3 
files.

```
scraper
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs

1 directory, 3 files
```

To build your project run the following. 

```sh
$ cargo build
   Compiling scraper v0.1.0 (file:///Users/foo/code/demo/scraper)
    Finished debug [unoptimized + debuginfo] target(s) in 0.98 secs
```

You can also execute the build as follows.

```sh
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/scraper`
Hello, world!
```


## Step 005: Using the hyper HTTP client

The wishlist is an HTML page delivered over HTTP. What is HTML? What
is HTTP?

To download over HTTP we use the _hyper_ _crate_. What is a crate?

Here's how to add the hyper dependency to Cargo.toml:

```

Add the hyper import with by writing `extern crate hyper;` in `main.rs`.

To make an HTTP request we need a hyper _http client_.

So this declarations go in your main function:

```rust
    let client = Client::new();
```

`let` _declares_ a client, which is set to the value created
by `Client::new()`.

`Client::new()` is a function on the `Client` _type_ that creates a
new `Client`. Every value in Rust has a type. We can talk about types
more later.

Next we are going to make an HTTP request to Amazon's server.

To do that we're going to write this series of lines:

```rust
    let mut response =
        client.get("https://brson.github.io/demo/wishlist.html")
        .send()
        .unwrap();
```

There's a lot going on here.

What we're looking at is a _chain_ of method calls. `client.get(...)`
is calling `get` on the client we just created. That's what the 'dot'
operator means - a method call. What are "methods"?

`send` and `unwrap` are also method calls. The details aren't important
right now, but feel free to read the documentation for those.

Also need to import `Client` to use it:

```rust
use hyper::Client;
```

## Step 010: Reading the response body

We'll not change the code to read the response body. You'll create a new mutable 
`String` variable. Add the following lines to the end of your main function.

```rust
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    println!("{:?}", body);
```  

That's how you use Hyper to read the HTTP response body into your program and 
print it. 

```sh
$ cargo build          
   Compiling scraper v0.1.0 (file:///Users/foo/code/demo/step010/scraper)
error: no method named `read_to_string` found for type `hyper::client::Response` in the current scope
  --> src/main.rs:14:14
   |
14 |     response.read_to_string(&mut body).unwrap();
   |              ^^^^^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
   = help: candidate #1: `use std::io::Read`

error: aborting due to previous error

error: Could not compile `scraper`.

To learn more, run the command again with --verbose.
```

The rust compiler tells us that it can't find the given method. It also tries to 
suggest a fix by adding `use std::io::Read` to our code. Try that fix and see if 
it works. Add the following line of code to the block of using statements at the 
top. This will import the necessary implementation for reading the response as a 
string. 

```rust
use std::io::Read;
```

Now you can build and run your program successfully. It will download the page and 
print it to the screen. 

## Step 3: Parsing the HTML

We'll be using the `select` library to find the data we care about on the 
wishlist. Add the following dependency to your `[dependencies]` section of your 
`Cargo.toml` file. 

```
select = "0.3.0"
```

Then you can use the `Document` parser in your rust function. Add the following 2 
statements to your `main.rs`. 

```rust
use select::document::Document;
use select::predicate::{Class};
```

The `select` library is used to find HTML objects by _class_, _id_, or _name_. We 
can see that the products are all contained in an HTML div with a class `a-row`. 
We can get a list of all `a-row` HTML elements using the following code snippet.  

```
    let document = Document::from(body.as_str());
    let rows = document.find(Class("a-row"));
```

Iterating to print the contents of each row:

```
    for row in rows.iter() {
        println!(" * Row {}", row.text());
    }
```

Step 015: Finding the Product Name and price

The actual elements containing the product name and price are somewhere in the rows
we printed last time. Running your program in the current state will also print out 
several rows that don't contain any products so these will also have to be filtered 
out.

First we can see that product names are all contained in `<h5>` elements. We can 
find the names with the following snippet.

```rust
        let maybe_title_node = row.find(Name("h5")).first();
```



```rust
        let maybe_price_node = row.find(Class("a-color-price")).first();
```