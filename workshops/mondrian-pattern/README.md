Mondrian Pattern Generator
==========================

<a title="Piet Mondrian [Public domain], via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File%3ATableau_I%2C_by_Piet_Mondriaan.jpg"><img width="512" alt="Tableau I, by Piet Mondriaan" src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/36/Tableau_I%2C_by_Piet_Mondriaan.jpg/512px-Tableau_I%2C_by_Piet_Mondriaan.jpg"/>
</a>


Piet Mondrian (7 March 1872 - 1 February 1944) is propably best known for establishing a distinctive visual pattern in our cultural heritage. Its characteristic way of creating images by making subsequent, orthogonal subdivisions has inspired not only fasion designers and confectioners but also [computer scientists](https://github.com/qiyuangong/Mondrian).

<a title="By Eric Koch / Anefo (Nationaal Archief) [CC BY-SA 3.0 (http://creativecommons.org/licenses/by-sa/3.0)], via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File%3AMondriaanmode_door_Yves_St_Laurent_(1966).jpg"><img width="30%" alt="Mondriaanmode door Yves St Laurent (1966)" src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/0f/Mondriaanmode_door_Yves_St_Laurent_%281966%29.jpg/512px-Mondriaanmode_door_Yves_St_Laurent_%281966%29.jpg"/></a>
<a title="By Heidi De Vries [CC BY 2.0 (http://creativecommons.org/licenses/by/2.0)], via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File%3AMondrian_Cake.jpg"><img width="30%" alt="Mondrian Cake" src="https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Mondrian_Cake.jpg/512px-Mondrian_Cake.jpg"/></a>

All Images from: [Wikimedia Commons](https://commons.wikimedia.org/)


Parallel processing is one of the reasons why rust was develped in the first place. Subdivide a canvas to be painted by multiple processes simultaneously is the the central idea of this workshop.

TODO: Quick overview for for absolute beginners
-----------------------------------------------

* Image generation as information processing
* Operations and data structures
* Sngle/Multi threadded computing
    * Processor
    * Shared ressources (The canvas)

TODO: Links to more detailed instructive minimals.

Implementation Outline
----------------------

1) root process
* sets up a canvas
* based on given parameters/settings
* subdivides canvas
* initiates subprocesses
2) each subprocess
* handles a part of the canvas
* makes further subdevisions
* may check on neighbours and negotiate with them [subdivisions and colours](mondrian-rules_avoid-unlucky-encounters.png) etc
3) negotiation with neighbours to avoid unlucky encounters
* requires inter-process communication

![unlucky encounters](mondrian-rules_avoid-unlucky-encounters.png)

Ownership of Canvas Regions
---------------------------

* The root thread will `borrow` parts of the canvas to other threads which will then paint it.

  * For an easy example we do not have to borrow but can simply hand over a reference to a part of the canvas, which will then be painted by the subordinate threads.

  * If we want to post-process the results we may need borrowing, indeed.

* The superordinate thread will connect neighbouring regions by establishing `channel`s so that the subthreads can negotiate colours and split positions
* Overlaps, which do not happen to be exactly mondrianish, make a case for shared resources and `locking` (so that we do not overpaint in an uncontrolled manner)
* [cf. Fearless Concurrency](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)
* [cf. multiple mutable slices of one vector](https://coderwall.com/p/w1yuza/take-two-or-more-mutable-slices-from-a-vector-in-rust)

Datastructure
-------------
* Patches or areas should be something like two-dimensional `slices`
* They may be of a more abstract kind in order to later get painted or rendered
* On the other hand, having truely parallel rendering would be nice!
* [cf. Slices](http://rustbyexample.com/primitives/array.html)
