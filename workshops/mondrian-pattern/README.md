Mondrian Pattern Generator
==========================

First sketch
------------

1) root process
* sets up a canvas
* based on given parameters/settings
* subdivides canvas
* initiates subprocesses

2) each subprocess
* handles a part of the canvas
* makes further subdevisions
* may check on neighbours and negotiate with them [subdivisions and colours](mondrian-rules_avoid-unlucky-encounters.png) etc

3) negotiation with neighbours
* requires inter-process communication

Ownership of Canvas Regions
---------------------------

* The root thread will `borrow` parts of the canvas to other threads which will then paint it.

  * For an easy example we do not have to borrow but can simply hand over a reference to a part of the canvas, which will then be painted by the subordinate threads.

  * If we want to post-process the results we may need borrowing, indeed.

* The superordinate thread will connect neighbouring regions by establishing `channel`s so that the subthreads can negotiate colours and split positions
* Overlaps, which do not happen to be exactly mondrianish, make a case for shared resources and `locking` (so that we do not overpaint in an uncontrolled manner)
* [cf. Fearless Concurrency](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)

Datastructure
-------------
* Patches or areas should be something like two-dimensional `slices`
* They may be of a more abstract kind in order to later get painted or rendered
* On the other hand, having truely parallel rendering would be nice!
* [cf. Slices](http://rustbyexample.com/primitives/array.html)
