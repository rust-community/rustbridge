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

to create a new blank rust project, and run `cargo run` in the directory
to make sure that it builds a hello world program.





