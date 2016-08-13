
# Project

This project is a URL shortener (think Bitly) designed and build in such a
way that it serves as a well documented example of how to design and
build a Rust project.

The URL shortener will:

* Take requests for shortened URLs (e.g. shrt.com/abcd)
and will redirect the user to a longer, ugly URL.
  * Count and persist the number of redirects.
* Provide a simple web interface to add URLs to the database.
  * Validate that submitted URLs are valid URLs.

We'll need:

* Web server & client : We'll use Nickel because Iron requires Rust Nightly
  and we'd like to be on Stable...
* Database : SQLite because it's simple to install and we don't really need
  proper relational features.
* Light web interface : 

URL shortening will be done using the following algorithm:

* Each URL will be assigned an auto-incrementing ID in the database.
* When a request is received, the short-ID is converted to the database
  ID by transforming it using rustc\_serialize::Base64.  This is not
  as efficient as a custom ser-des, but it easy.

# Data Design Notes

We'll record:

* Link : every long URL that is to be shortened.
* LinkUse : every case of a shortened URL being used.

# Web Interface

The server will respond to:
* /admin/\* : admin templates and functionality.
* All others with Base64 decoding.

# 
