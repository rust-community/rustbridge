[back to the Part I](part1.md)

-----------



Mondrian Pattern Generator (Part II)
====================================

Rust was developed to ensure certain safety principles of organising a multithreaded application. The final code example of Part I is already multi-threaded. The mondrian pattern makes a good case for studying parallel processing. The canvas should be painted with a valid mondrian pattern. Multiple, (non-overlapping) parts of the canvas can be painted in parallel without breaking the "mondrian principles".

### Why neighbours should be aware of each other:

+ avoid unlucky encounters of subdivisions
+ make colours conbine nicely on both sides of subdivisions


![unlucky encounters](mondrian-rules_avoid-unlucky-encounters.png)


-----------

[back to the Part I](part1.md)
