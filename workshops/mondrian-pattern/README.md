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

