[Back to the introduction](./mondrian-pattern/README.md) | [Continue to Part II](./mondrian-pattern/part2.md)

-----------


Mondrian Pattern Generator (Part I)
===================================


What to learn in this part?
---------------------------

+ Invoke Operations (Alias: Call Functions)
+ Variables
+ When invoking an operation, specify how it should do its job in detail (Alias: Arguments/Parameter variables)
+ Define new Operations (Alias: Function definitions)
+ How to read rust error messages
+ Let Operations invoke other operations in non-trivial combinations


Exercises
---------


### Exercise 0: Start with a prepared minimal Mondrian pattern

![](./mondrian-pattern/images/fig00_exercise-start.jpg)

At this point, you have executed a minimal `Hello World` rust project already! If not, please go [back to the introduction](./mondrian-pattern/README.md).
Now, replace that first project with the minimal Mondrian pattern generator:

&#9654;&#9654; Open the files `main.rs` (in the subfolder `src`) and `Cargo.toml` in your text editor.

If you are unsure where to find the files and/or how to open them in a text editor there are [very detailed instructions in Step 2 and 3 here](https://github.com/broesamle/RustWorkshop/blob/master/minimals/countinghands.md#step-2-where-is-the-program).

&#9654;&#9654; Clicking the links will show the file content in the browser window.

+ [new main.rs](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/src/main.rs)

+ [new Cargo.toml](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/Cargo.toml)

What you see looks quite complex and like a foreign language. True, it is rust, the language you will learn now! At this point, you don't need to understand it. You just have to copy and paste it :-)

![](./mondrian-pattern/images/main-rs_in-browser.jpg)

&#9654;&#9654; Copy and paste it from there into the file in the text editor. (Do it for both files in the same way.)

&#9654;&#9654; Save the files in your editor.

#### [Testing]

&#9654;&#9654; Make sure your machine has an internet connection working.

&#9654;&#9654; To execute the program do again `cargo run`.

Your rust compiler will work for some time now

&#9654;&#9654; Then, see what happens: can you see a very minimal Mondrian pattern (a red rectangle with a black frame) in a window?

In windows, it happens that the window is hidden behind other open windows. [TODO: Fix that!]

You should see something similar to this:
![](./mondrian-pattern/images/console+mondrianwindow.jpg)

Congratulations! You have made a project, adapted it so that it matches an example, and executed it successfully.

#### [Explanation]

Although the Mondrian pattern in the window is what we're here for, it is worth looking at the console window as well. Don't worry about the warnings for now. Rust tells us that there are things in the program that are never used. We will use them soon!


### Exercise 1: Make one vertical split

![](./mondrian-pattern/images/fig01_onevertical.jpg)

It is now time to have a closer look at the rust code. It is located in `src` folder and the file is `main.rs`.

&#9654;&#9654; Open the file `main.rs` located in folder `src` in your editor.

Most of the code you will never touch. Only the highlighted areas are important for you.

![](./mondrian-pattern/images/code_highlight-learners-area.jpg)

Furthermore, you only need to make tiny modifications at first:

&#9654;&#9654; Look at the upper highlighted area. Change the line <br>`paint_rectangle(20.0, 20.0, 300.0, 250.0, chn)` into <br>`vsplit_and_paint(20.0, 20.0, 300.0, 250.0, chn)`.

#### [Testing]

&#9654;&#9654; To execute the program do again `cargo run`.

Can you see the window open with the two-rectangles-pattern, as given in the sketch?


#### [Snapshot] 1
From time to time there will be such "snapshots": What are the exact changes in the code. The second link gives _a snapshot_ of how the code should look at this point.

[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/mondrian_exercise-snapshots/workshops/src/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0001-One-vertical-split-1-2-1-2.rs)


#### Explanation
The line you have changed is the line that _invokes_ the Mondrian painting activity. Technically speaking this line tells rust to _invoke an operation_, named `paint_rectangle`. Before, you have changed that line so as to execute a different operation, `vsplit_and_paint`.


### Exercise 2: Split position

![](./mondrian-pattern/images/fig02_onevertical-1third-2thirds.jpg)

Line 64 calculates the position where to split the canvas into two areas:

```rust
let splitpos = width / 2.0;
```

&#9654;&#9654; Replace the `2.0` with a `3.0`.

#### [Testing] --as usual--

#### [Snapshot] 2
[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/master/src/workshops/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0002-One-vertical-split-1-3-2-3.rs)


#### Explanation

The first highlighted area, the one you have modified in _Exercise 1_ executes the operation `vsplit_and_paint`. The second highlighted area _defines_ the operations `vsplit_and_paint` and `paint_rectangle`.
An operation is defined as a sequence of (other) operations to be executed on invocation.

Defining an operation can be compared to writing a recipe, invoking it means to actually prepare the dish _one time_. This can be repeated any number of times even though there is only one definition/recipe.

What you have done in this exercise is to modify the operations that are executed, whenever `vsplit_and_paint` is executed. In this case you changed the ratio between the left and the right side of the split to _one third:two thirds_.


### Exercise 3a: Specify the colour for `paint_rectangle`

![](./mondrian-pattern/images/fig03_colour-as-parameter.jpg)

&#9654;&#9654; Find the lines where `paint_rectangle` is defined:

```rust
fn paint_rectangle(x :f64, y :f64, width :f64, height :f64, chn: SendChannel)
{
    println!("paint_rectangle: {:}, {:}, {:}, {:}", x, y, width, height);
    chn.send( ([x, y, width, height], RED) ).unwrap();
}
```

The stuff in the `( . . . )` is called _parameters_, separated by `,`. Parameters are used to specify the details, how an operation should be executed in detail. This exercise wants to _specify the colour for paint_rectangle_. Here we go!

&#9654;&#9654; Add an extra parameter `c` to the definition of `paint_rectangle` like this:

```rust
fn paint_rectangle(x :f64, y :f64, width :f64, height :f64, c: types::Color, chn: SendChannel)
```

#### Testing

&#9654;&#9654; Save the file.

&#9654;&#9654; `cargo run`

**Bam! Your first rust build-error!**

![](./mondrian-pattern/images/console_paintrect-parameter-error.jpg)


#### Explanation

They look scary in the beginning!

![](./mondrian-pattern/images/build-error_explained.jpg)

We added one little thing in the definition of an operation and we get this amount of error!

This is what rust is complaining about: `this function takes 6 parameters but 5 parameters were supplied`. Makes sense. We changed the operation and now it requires more information to be executed: The colour of the rectangle. The error occurs because this information is required by the operation but it is not provided.

Where should it be given? ...at each point in the code where the execution of `paint_rectangle` is invoked: Lines 67 and 72; exactly the lines where the two (identical) errors are reported. Makes sense.


### Exercise 3b: Different colours on each side

![](./mondrian-pattern/images/fig04_two-different-colours.jpg)

The issue encountered in Exercise 3 is that colour information is required on the one hand but not provided. When fixing this, we will (as a side effect) have a Mondrian pattern with different colours on each side!

Where do the error messages direct you?

```
src/main.rs:67:9: 67:57 error: this function takes 6 parameters but 5 parameters were supplied [E0061]
. . .
src/main.rs:72:9: 72:73 error: this function takes 6 parameters but 5 parameters were supplied [E0061]
```
Lines 67 and 73:

```rust
paint_rectangle(x, y, splitpos, height, chnleft)
. . .
paint_rectangle(x+splitpos, y, width-splitpos, height, chnright)
```
Both lines invoke `paint_rectangle` with slightly different parameters.

&#9654;&#9654; In each line, behind the parameter `height`, the colour has to be provided, i.e. like this:

```rust
paint_rectangle(x, y, splitpos, height, RED, chnleft)
. . .
paint_rectangle(x+splitpos, y, width-splitpos, height, BLUE, chnright)
```

#### [Testing]
Testing this via `cargo run` should make rust happy again and it runs the example. Yeah! But both sides are red hmmm.

And the warnings have changed:
```
src/main.rs:17:1: 17:48 warning: constant item is never used: `GREEN`, #[warn(dead_code)] on by default
src/main.rs:17 const GREEN:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];
               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/main.rs:19:1: 19:48 warning: constant item is never used: `YELLOW`, #[warn(dead_code)] on by default
src/main.rs:19 const YELLOW:  [f32; 4] = [1.0, 1.0, 0.0, 1.0];
               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/main.rs:78:61: 78:62 warning: unused variable: `c`, #[warn(unused_variables)] on by default
src/main.rs:78 fn paint_rectangle(x :f64, y :f64, width :f64, height :f64, c: types::Color, chn: SendChannel)

```

#### Explanation

Yes, there is still two colours unused, that's fine. But there is an `unused variable`, `c` in line 78.

```rust
fn paint_rectangle(x :f64, y :f64, width :f64, height :f64, c: types::Color, chn: SendChannel)
```

This is the line where you added the new parameter for colour. Rust calls this a _variable_ because parameters are special sorts of variables.
What is a variable? A variable is a container that can carry some value during program execution. To make things easier for you, you can give names to these containers, here `c` for colour. We decided to give `paint_rectangle` some information about the colour to be used and we do that via the parameter variable c.

Rust was happy because all required parameters are now given again. It was happy enough to make and run the program but it was also a bit suspicious because there is c, which is never used (remember the warning).

The last thing we have to adapt the definition of `paint_rectangle` so as to use colour information. We just need to get the information from `c` into the painting of the rectangle. The actual painting of the rectangle is invoked here:
```rust
chn.send( ([x, y, width, height], RED) ).unwrap();
```


&#9654;&#9654; **Final Thing to do:** Replace `RED` with `c` and give it should all be working as planned!

#### [Snapshot] 3
[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0003-Add-parameter-c-Color-to-paint_rectangle.rs)


### Exercise 4: Random split position

![](./mondrian-pattern/images/fig05_random-splitpostition.jpg)

This Exercise will give the Mondrian painter a bit of 'artistic' (!?!) freedom. It can randomly choose the position of where to split the canvas. You will modify the operation `vsplit_and_paint` to use a random number for the split position.

A couple of things are already prepared in the example code.
There is an operation for generating random numbers available: For instance, `rng.gen_range(0.0, 20.0)` will produce a random number between `0.0` and `20.0`.

Currently it is deactivated in the example; it is placed in a comment _comment_:
```
// the remainder of a line after a double forward slash will be ignored

/*
Furthermore, everything between 'slash star' and
'star slash' is also ignored, even over multiple lines.
*/
```

&#9654;&#9654; Uncomment lines 7 and 61 by removing the dashes:

```rust
use rand::Rng;
. . .
let mut rng = rand::thread_rng();   //init a random number generator`.
```

&#9654;&#9654; Run the example in order to know whether rust is still happy with the changes. (It should run as before except for two more warnings, which is not a problem right now.)

From _Exercise 2_ you already know where the split position is calculated.

&#9654;&#9654; Find the relevant line in the code again and replace it with

```rust
let splitpos = rng.gen_range(0.0, width);
```

#### [Testing]
You will have to run the example a couple of times in order to see the different split positions across several runs.

#### Explanation
`vsplit_and_paint` receives a number of parameters which define the position and dimensions of where to paint the Mondrian pattern: x, y coordinates of the upper left corner and width and height of the canvas (and the colour and some technical stuff).

It decides on a position where to split the canvas vertically and calculates two smaller areas, left and right of that split so as to fully cover the canvas. Technically speaking, the split position is the width of the left side. The width of the right side is the width of the whole canvas minus the width of the left side. Similar calculations are done for the x position of the right hand part.

The next step is to delegate the two parts to some other operation(s) that will take care of each side. This happens by invoking `paint_rectangle` with according coordinates.

**If you are confused, making a sketch of the subdivision will help you to understand what is going on in detail!**


#### [Snapshot] 4
[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0004-Randomise-the-split-position.rs)


### Exercise 5: Horizontal split

![](./mondrian-pattern/images/fig06_horizontal-split.jpg)

You will _define an operation_ that works in the same way as `vsplit_and_paint` except that it does Horizontal splits.

&#9654;&#9654; If you have not made a sketch of the vertical canvas split before, now you will need it.

&#9654;&#9654; You will also need one for a horizontal split.

&#9654;&#9654; Make a copy of the operation definition of `vsplit_and_paint` and rename it to `hsplit_and_paint`.

&#9654;&#9654; Identify the code lines where the height and the y-coordinates are calculated.

+ this will involve all lines that touch `splitpos`, `width` and `x`

+ `height` and `y` have to be calculated depending on the split position

+ Remember the order of the parameters for `paint_rectangle`: `x, y, width, height, ...`. You have to give the new coordinates in the exact same order!

+ Take some time, it is not trivial!

#### [Testing]
The new operation has to be invoked at some point:

&#9654;&#9654; For testing it, find the spot where `vsplit_and_paint` is invoked and replace it with `hsplit_and_paint`.

&#9654;&#9654; Run the example.


If you get error messages, most likely, you have done smaller errors. Finding and fixing errors is one of the key skills in programming.

&#9654;&#9654; If you feel adventurous you should try to find them yourself! Otherwise, you might need some smaller advice to continue by yourself.

Copying the snapshot is the last option. If you do so, please take some time to figure out, what is going on!

&#9654;&#9654; Feel free to use more interesting colours :-)

#### [Snapshot] 5
[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0005-Horizontal-split.rs)


### Exercise 6, 7: Combine split and delegate operations
`paint_rectangle(x, y, splitpos . . .` and `paint_rectangle(x+splitpo, y, . . .`.

It is now time to play with the new operations!

&#9654;&#9654; If you fell adventurous, try to figure out for yourself how to achieve one of the new patterns.


**In more detail:** Currently, `hsplit_and_paint` triggers `paint_rectangle` for both sides of the split. For the first pattern

&#9654;&#9654; Modify `vsplit_and_paint` so that it triggers `paint_rectangle` on the left side of the split and `hsplit_and_paint` on the right side.

&#9654;&#9654; Don't forget to adapt the initially invoked operation in line 53.

![](./mondrian-pattern/images/fig07_vertical+horizontal-split.jpg)

&#9654;&#9654; Modify `hsplit_and_paint` so that it triggers `paint_rectangle` on the left side of the split and `hsplit_and_paint` on the right side.

&#9654;&#9654; Don't forget to adapt the initially invoked operation in line 53.

![](./mondrian-pattern/images/fig08_vertical+2horizontal-splits.jpg)

#### [Snapshot] 6
[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0006-One-vertical-and-one-horizontal-split.rs)

#### [Snapshot] 7
[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0007-One-horiz.-two-vert.-splits-above-and-below.rs)



### Exercise 8: An infinite pattern generation process

![](./mondrian-pattern/images/fig09_nontricial-nested.jpg)

**Warning!** Some changes here may crash the program or make your machine unresponsive. That's okay and it's a natural part of learning programming. You can just close the program and/or restart your machine and continue.
Save your data before you run.

There is one way of achieving this pattern by defining more specific operations. However, things will be much more interesting if you

&#9654;&#9654; Try to modify and recombined the existing operations, only.

&#9654;&#9654; Make a diagram of the pattern you want to achieve. Sketch out step-by-step all the operations that will be invoked and in what order. For example, for the first pattern in Exercise 7 this will be:

* `vsplit_and_paint` invokes for
    * left side: `paint_rectangle`
    * right side: `hsplit_and_paint` invokes for
        * top side: `paint_rectangle`
        * bottom side: `paint_rectangle`

The _split_ operations invoke to others, which actually run in parallel! The pattern painting process is artificially delayed so that you can see that in detail.

#### [Snapshot] 8
[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0008-Fail-in-a-funny-way-infinite-recursion.rs)


### Exercise 9: Avoiding the infinite pattern generation process.

**TODO!!**

#### [Snapshot] 9
[view changes](https://github.com/rust-community/rustbridge/commit/6a51b31a396e7a9200ac21a0cc5ab574915a3b2f)
|
[main.rs snapshot](https://raw.githubusercontent.com/rust-community/rustbridge/master/workshops/src/mondrian-pattern/codebase/mondpaint/src/snapshnots/main_0009-Controlled-recursion.rs)


-----------

[Back to the introduction](./mondrian-pattern/README.md) | [Continue to Part II](./mondrian-pattern/part2.md)
