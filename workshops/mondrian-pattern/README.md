Mondrian Pattern Generator
==========================

<a title="Piet Mondrian [Public domain], via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File%3ATableau_I%2C_by_Piet_Mondriaan.jpg"><img width="512" alt="Tableau I, by Piet Mondriaan" src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/36/Tableau_I%2C_by_Piet_Mondriaan.jpg/512px-Tableau_I%2C_by_Piet_Mondriaan.jpg"/>
</a>


Piet Mondrian (7 March 1872 - 1 February 1944) is propably best known for establishing a distinctive visual pattern in our cultural heritage. Its characteristic way of creating images by making subsequent, orthogonal subdivisions has inspired not only fasion designers and confectioners but also [computer scientists](https://github.com/qiyuangong/Mondrian).

<a title="By Eric Koch / Anefo (Nationaal Archief) [CC BY-SA 3.0 (http://creativecommons.org/licenses/by-sa/3.0)], via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File%3AMondriaanmode_door_Yves_St_Laurent_(1966).jpg"><img width="30%" alt="Mondriaanmode door Yves St Laurent (1966)" src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/0f/Mondriaanmode_door_Yves_St_Laurent_%281966%29.jpg/512px-Mondriaanmode_door_Yves_St_Laurent_%281966%29.jpg"/></a>
<a title="By Heidi De Vries [CC BY 2.0 (http://creativecommons.org/licenses/by/2.0)], via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File%3AMondrian_Cake.jpg"><img width="30%" alt="Mondrian Cake" src="https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Mondrian_Cake.jpg/512px-Mondrian_Cake.jpg"/></a>

**All Images from:** [Wikimedia Commons](https://commons.wikimedia.org/)


Information Processing
----------------------

The universe is incredibly complex. When it comes to information processing technology, the first step to solving a problem is to understand the problem in a precise way.

What am I interested in?
What am I _not_ interested in?
Which information do I have?
Which information do I need?
How can I derive or _create_ the information I need based on the information I have.

The mondrian universe
---------------------

![](images/mondrian-associations.jpg)

**Image from:** [Visuwords (TM)](http://visuwords.com)

Not incredibly complex, yet, too complex and vague for a straight-forward information processing problem.


### What to focus on?
![](images/mondrian-associations-focus.jpg)

**What are we not interested in?** (At least not for our little project here.) His biography. The techniques of painting. And so on.

**What are we interested in?** The paintings of Piet Mondrian. They are oil paintings on canvas. The canvas is made of some fabric (linen) which is a plant... The paint is some colourful substances mixed with oil as a suspension -- **STOP!** We could continue like this for ever.

**What aspect(s) of his paintings are we interested in?** The distinctive visual patterns. For us as humans all these things are often immediately clear but for the machines we want to program it is not.


### Mondrian Patterns

What makes this type of pattern _this_ type of pattern?

With this description we will certainly make an over-simplification, omitting many aspects of the choices he made regarding composition, geometry, colour... We will see more of the complexity the further we proceed.

* We see the _canvas_ as a rectangular surface that has a colour at each location. Initially it is white or some other more or less uniform colour.

* The Mondrian Painting is a canvas filled with coloured rectangular areas bounded by black borders.

This is (still) not precise enough to define the distinctive pattern we are interested in. It could also mean something like [random rectangles](https://williamaadams.wordpress.com/2013/12/12/multitask-ui-like-its-1995/).

Up to now, nothing in our _definition_ makes sure the rectangles

* are aligned with the edge of the canvas,
* cover the whole canvas,
* do not overlap, producing non-rectangular remainders.

TODO: Add more odd examples

**Definition:** With _Mondrian Pattern_ we will refer to a rectangular area, the _canvas_, that is filled _completely_ with differently _coloured rectangles_. The rectangles are oriented 'upright' so that their edges are aligned with the borders of the canvas.


Processing Mondrian Patterns
----------------------------

We know _what_ we want to achieve but we do not now _how_ we can achieve it.

TODO: Most simple example: One filled Rectangle

TODO: Second most simple example: One Subdivision

TODO: Complex example: More subdivisions


To fill a given rectangular _canvas_ with a pattern that satisfies the criteria of the definition we can use these steps:

1. Make vertical/horizontal subdivisions so that rectangular sub-regions emerge.
2. Paint each sub-region with a coloured rectangle and paint a black border around it.

There are other ways of achieving similar or the same type of pattern. Also, we are far from a level of precision that would suit programming a machine to do it. But for us, we have gained a lot of precision already.


Exercises
---------

### TODO: Create clean step by step instructions. . .
. . . that go thogether with the code examples in https://github.com/rust-community/rustbridge/commits/mondrian_learners-code-examples/workshops/mondrian-pattern/codebase/mondpaint/src


* Make a new project (TODO: link to installfest instructuins)
* Replace main.rs and Cargo.toml with the following starting point (TODO: link)
* Execute the program and make sure you see a minimal mondrian pattern, i.e. a red rectangle.

Error message after adding an extra parameter to `paint_rectangle`:
```
Compiling mondpaint v0.1.0 (file:///home/broe/projets/rustbridge/workshops/mondrian-pattern/codebase/mondpaint)
src/main.rs:80:9: 80:57 error: this function takes 6 parameters but 5 parameters were supplied [E0061]
src/main.rs:80         paint_rectangle(x, y, splitpos, height, chnleft)
                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/main.rs:80:9: 80:57 help: run `rustc --explain E0061` to see a detailed explanation
src/main.rs:80:9: 80:57 note: the following parameter types were expected: f64, f64, f64, f64, [f32; 4], std::sync::mpsc::Sender<([f64; 4], [f32; 4])>
src/main.rs:85:9: 85:73 error: this function takes 6 parameters but 5 parameters were supplied [E0061]
src/main.rs:85         paint_rectangle(x+splitpos, y, width-splitpos, height, chnright)
                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/main.rs:85:9: 85:73 help: run `rustc --explain E0061` to see a detailed explanation
src/main.rs:85:9: 85:73 note: the following parameter types were expected: f64, f64, f64, f64, [f32; 4], std::sync::mpsc::Sender<([f64; 4], [f32; 4])>
error: aborting due to 2 previous errors
error: Could not compile `mondpaint`.
```

For Exercice 5 (?) we will use random numbers to make things more interesting.

A coupple of things are already prepared in the example, but currently deactivated by placing them in a _comment_:

```
// the remainder of a line after a double forward slash will be ignored

/*
Furthermore, everything between slash star and
star slash is also ignored, even over multiple lines
*/
```

* Uncomment lines 15: `use rand::Rng;` and 74 `let mut rng = rand::thread_rng();   //init a random number generator`

Now there is a new operation for generating random numbers available: `rng.gen_range(0.0, 20.0)` will produce a random number between `0.0` and `20.0`.

* Modify the operation `vsplit_and_paint` to generate a random split position. The value to be used as split position is calculated in line 76 `let splitpos = rng.gen_range(0.0, width);` Use the operation we just activated `rng.gen_range(...)`.


After defininf the new operation `vsplit_and_paint`, build output should look like this:
```
Compiling mondpaint v0.1.0 (file:///home/broe/projets/rustbridge/workshops/mondrian-pattern/codebase/mondpaint)
src/main.rs:23:1: 23:48 warning: constant item is never used: `GREEN`, #[warn(dead_code)] on by default
src/main.rs:23 const GREEN:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/main.rs:25:1: 25:48 warning: constant item is never used: `YELLOW`, #[warn(dead_code)] on by default
src/main.rs:25 const YELLOW:  [f32; 4] = [1.0, 1.0, 0.0, 1.0];
            ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/main.rs:91:1: 107:2 warning: function is never used: `hsplit_and_paint`, #[warn(dead_code)] on by default
src/main.rs:91 fn hsplit_and_paint(x :f64, y :f64, width :f64, height :f64, chn: mpsc::Sender<(Rectangle, Color)>) {
            ^
 Finished debug [unoptimized + debuginfo] target(s) in 2.88 secs
```

* Time to test the new operation! Currently, `hsplit_and_paint` triggers `paint_rectangle` for both sides of the split: lines 79 (?) and 85 (?): `paint_rectangle(x, y, splitpos . . .` and `paint_rectangle(x+splitpo, y, . . .`. Modify `hsplit_and_paint` so that it triggers `paint_rectangle` on the left side of the split and `hsplit_and_paint` on the right side.

TODO: finalise instructions as sketched-out here

#### Exercise 8:

Option 1: Introduce more special operations
Option 2:
Try to use only the operations we have defined so far -- you will fail in a funny way!
;-)

![](images/exercises-a.jpg)
![](images/exercises-b.jpg)

TODO: **Exercise 9:** Avoiding infinite recursive patterns.



Beyond this Workshop
--------------------

### Why neighbours should be aware of each other

Avoid unlucky encounters of frames and colours.

![unlucky encounters](mondrian-rules_avoid-unlucky-encounters.png)

TODO: Links to more detailed instructive minimals.
