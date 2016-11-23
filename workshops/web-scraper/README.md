# Web Scraper Workshop

This workshop guides participants through creating a webscraper to get data out of an Amazon wishlist using the [hyper] crate and the [select] crate.

[hyper]: https://crates.io/crates/hyper
[select]: https://crates.io/crates/select

## Setup Instructions

In case internet isn't reliable, serve `doc/wishlist.html` from a local
computer and anywhere that you see
`https://brson.github.io/demo/wishlist.html`, use your local URL instead.

In the `doc` directory, there's a script you can use to walk through the steps
in `demo.md`, as well as slides using [Remark] in `slides/project.html`. Feel
free to edit the content of the slides by changing the markdown in the textarea
in `project.html`. There are speaker notes; to use them, open index.html in a
browser and type 'c' to create a "cloned" view of the slideshow in another
window. Then in one of the windows, type 'p' to switch to presenter view. The
two windows will now change slides together.

[Remark]: https://remarkjs.com

### Test on Windows!

Definitely run through the project on a Windows machine or VM before running
the workshop. In Oct 2016, we had issues with linking to openssl on Windows
machines in order for hyper to be able to make https requests, but check to see
if [hyper has switched to some other solution][winapi]. The solution we came up
with eventually was to [disable openssl on windows][disable] by putting this in
`Cargo.toml`:

```toml
[dependencies.hyper]
version = "0.9"
default-features = false
```

[winapi]: https://github.com/hyperium/hyper/issues/573
[disable]: https://github.com/hyperium/hyper/issues/785#issuecomment-217927498
