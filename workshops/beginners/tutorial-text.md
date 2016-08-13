Image generation
================

We're going to use Rust to build an image-generation program. This program will
automatically generate images in the style of Ellsworth Kelly's painting
[Spectrum Colors Arranged by Chance](https://www.sfmoma.org/artwork/99.352) -
that is to say, an image made up of a bunch of brighly-colored squares
arrainged in a random pattern. This is a fairly straightforward programming
exercise that you could do in many different programming languages, and we're going
to teach you how to use Rust to do it.

What you already know
---------------------

For this tutorial, we are assuming that you've already written and successfully
compiled some Rust programs before, and that you understand how to use cargo to
create a bare Rust project. If you haven't, try taking a look at tutorials
like {link to tutorial} first before attempting this one.


Step 1 - create an image
=========================

The first step to writing a program that will generate a random image
is writing a program that will generate an image at all. Go ahead and run

`cargo new --bin image-generator`

to create a new blank Rust project, and run `cargo run` in the directory
to make sure that it builds a hello world program.


Using external crates
------------------------

In Rust, external libraries are referred to as *crates*. You can import an
external crate into your project to use someone else's code to accomplish a
task.  For the image generator, we're going to use the
[image](https://github.com/PistonDevelopers/image) crate to work with image
files.

In your Cargo.toml file, right below the `[dependencies]` line, type `image =
"*"`. The Cargo.toml file contains metadata about a Rust project, and was
automatically created for you when you ran `cargo new`. It probably already has
some lines that indicate what the name of the project is and who the authors
are, which are useful when you share your code with other people. One of the
other jobs of Cargo.toml is to indicate which external crates your project is
using, so cargo can download them from a repository when it compiles your
program.

`image = "*"` means that your project requires the `image` crate, and
TODO: rewrite the code to use a specific versoin of image and explain why we care

At the top of your `src/main.rs` file, type

`extern crate image`

This line of code indicates that you want to include the image crate in this file, and
you can access code provided by the image crate by prefixing it with `image::`.

The next time you run `cargo run`, you should see cargo take some time to download
the image crate before it compiles your code and runs the default hello program.
Even though we're not using any of the functionality in image yet, we're including
it in our project, so cargo needs to fetch the code in order to successfully compile it.





