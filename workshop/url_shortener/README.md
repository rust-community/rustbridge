
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

* Web server & client
* Database
* Light web interface

URL shortening will be done using the following algorithm:
* Each URL will be assigned an auto-incrementing ID in the database.
* When a request is received, the short-ID is converted to the database
  ID by transforming it from pseudo-base64 [a-zA-Z0-9 -\_] and back to binary.

This should extend rustc\_serialize.

# Data Design Notes


# Libraries

# 
