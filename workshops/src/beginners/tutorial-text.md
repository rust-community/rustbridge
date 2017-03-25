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


Goal 1 - a plain red square
===========================

The first step to writing a program that will generate a random image is
writing a program that will generate any image at all. We're going to generate
an image that is plain red with nothing else (the exact color doesn't matter,
the important thing is that there's no complexity, it's just one color).

Go ahead and run

`cargo new --bin image-generator`

to create a new blank Rust project, and run `cargo run` in the directory
to make sure that it builds a hello world program.


Using external crates
------------------------------

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

`extern crate image;`

This line of code indicates that you want to include the image crate in this file, 
you can access code provided by the image crate by prefixing it with `image::`.

The next time you run `cargo run`, you should see cargo take some time to download
the image crate before it compiles your code and runs the default hello program.
Even though we're not using any of the functionality in image yet, we're including
it in our project, so cargo needs to fetch the code in order to successfully compile it.

Using standard library code
---------------------------

Rust comes built-in with a lot of code to do basic tasks that are relevant for almost all
software projects, such as opening and writing to files, or getting some information about
the operating system. Since this *standard library* code already comes built into Rust, we don't
need to include an external crate to make use of it. Instead, we use the `use` command:

Go ahead and type:

    use std::fs::File;
    use std::path::Path;

at the top of your main.rs, right below the `extern crate image;` line (note
the semicolons!).  This tells Rust that in your source file, you want to
include two pieces of standard library functionality.  `File` is a Rust struct
(we'll talk more later about what a struct is) that lets you do things
involving files, like creating them or writing to them. Since we want to create
an image file and write it to the filesystem, we'll need something like this.
`Path` is a rust struct that lets you manipulate filesystem paths (like
`C:/Users/` in Windows or `/usr/bin/` in a Unix-like operating system). We'll need this
to tell `File` where we should put the image file that we're going to generate.

Using the image crate
---------------------

Okay, now that we have our dependencies set up, it's time to actually make an
image happen!  Let's look at the entire contents of the `main()` function -
remember, when you run a Rust program, execution starts at the beginning of the
`main()` function, and the program quits when it gets to the end of `main()`.

    fn main() {
        let image_size = 400;

        let mut imgbuf = image::ImageBuffer::new(image_size, image_size);

        let red = [255, 0, 0];

        for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgb(red);
        }

        let ref mut fout = File::create(&Path::new("image.png")).unwrap();

        let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
    }

`let image_size = 400;` we decided (arbitrarily) that 400x400 pixels is a good
size for an image, so let's assign that number to a variable with a name that
makes sense. `let` is the keyword for assigning a value to a variable.

It's important that we end this line with a semicolon - some
programming languages use a newline to indicate that we are done with one
statement and will move onto the next one, but in Rust we use a semicolon for
this, and the newline isn't meaningful in and of itself.  Later on, we'll talk
about some cases where a line of code *doesn't* end in a semicolon, and why
that is, but for now you can think of a semicolon as the thing that separates one
statement from another in Rust.

`let mut imgbuf = image::ImageBuffer::new(image_size, image_size);` Remember
how we included the external crate image? Well, here's where we use it.
`image::ImageBuffer::new` is a function on the ImageBuffer struct within the
image crate to make a new, default image of a given width and height. How did
we know what function we needed to call to do this?  [The documentation for the
image crate says
so](http://www.piston.rs/image/image/struct.ImageBuffer.html#method.new).

And what's this `mut` business? In Rust, when you declare a variable with
`let`, it is *immutable*. You cannot change the value stored in the variable,
and your program will not compile if you try. Sometimes, however, you do want
to be able to change the value stored in a variable after you define it, and in
these cases you can declare the variable with `let mut` to indicate that you
want the variable to be *mutable*.

`let red = [255, 0, 0]`. Rust, like most programming languages, has a way to
talk about a group of several values. In Rust, this is called a *slice*. We
declare a slice by putting any number of comma-separated values in between `[
]`, and then we can treat the entire slice as a unit when we assign it to a
variable or pass it to a function.

It's very common in programming languages to represent a specific color as a
tuple of three numbers between 0 and 255, which represent the amount of pure
red, pure green, and pure blue in the color, in that order. [0, 0, 0] is
completely black, [255, 255, 255] is completely white, [128, 128, 128] is a
neutral grey, and [255, 0, 0] is red. And red is exactly what we want right
now. We'll use more interesting colors later.

Iterators and References
-----------------------

These next three lines of code aren't very long, but they introduce a number of concepts
that are pervasive in Rust. Let's take it slow:

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb(red);
    }

You're probably familiar with the concept of a for loop - *for* every item in a collection
of items, { *do something* with each item in turn }. `imgbuf.enumerate_pixels_mut()` returns a
value called an *Iterator*, and, put simply, an Iterator is something that you can loop over
in a for loop. In this case, `.enumerate_pixels_mut()` is a method we can call on an ImageBuffer
(remember, we created this ImageBuffer a few lines up). The for loop above will take a series of items
from an iterator and place the, one at a time, into the variable to the left `in`.

The iterator contains a list of fixed size items called a tuple. Each element in a tuple can have a
different type, for example an i32 and str. In the case of the `enumerate_pixels_mut()`, this
tuple has the coordinates of of the pixel in the first 2 items, and a mutable reference to the pixel
at those coordinates. This reference allows us to change the pixel directly.  The `(_, _, pixel)` allows
the tuple to broken into local variables; the `_` means that item in the tuple will be ignored. 

`let ref mut fout = File::create(&Path::new("image.png")).unwrap();`
Now that we've gone through every pixel in our image and set it to red, we're ready to save it to a file.
`Path::new("image.png")` creates a *path* relative to the current directory whose name is "image.png" - this is
the file we will save our image to, and that's why we had to `use std::path::Path`. `File::create()` takes a path
as an argument and creates a file there. Why the `&`? That creates a reference.

Errors and unwrapping
---------------------

`.unwrap()` is a very common idiom in Rust code. TODO explain more about unwrap() and when to use it and when not to use it



`let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);` Finally, now that we've prepared the `fout` file handler, we use
the image crate's `.save()` functionality to save it to a file. Our program is complete. 

Run the program with `cargo run`, and, if all went well, you should see a file in teh current directory called "image.png". If you
open it up in an image viewing program, you should see a completely red square.

It's sorta like "hello world" for images. Now, let's make some more interesting stuf!

Goal 2
======

