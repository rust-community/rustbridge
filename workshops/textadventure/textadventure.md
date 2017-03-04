# Text Adventure

For this workshop, let’s build a simple text adventure game in Rust.

Here is what we need:

**Game Board** - A 5 by 5 grid containing a total of 25 rooms. 

**Rooms** - A room may have up to 4 **Walls**. Each wall may be:
+ **Open** which will allow unrestricted passage through.
+ **Solid** which will prevent passage.
+ **Magical** which will allow passage only with a secret word.
+ Some rooms are dark and some are light. 
 
**Characters** - We will have 3 kinds of characters.
+ You the adventurous **Explorer**, wandering from room to room.
+ **Leprechauns** the good wee people with a mischievous sense of humour. They can give you gold or **Magic Words** but you never know if they are telling the truth. They can teleport to any room.
+ **Gnomes** who guard the treasures of the earth and just might take your stuff if you’re not careful. They can go one space in any direction - they know the magic word to walk through any kind of wall. 
 
**Things**
+ **Food** gives you **Energy** to keep going.
+ **Gold** pays the way.
+ **Pyrite** is given to you by mischievous leprechauns - also good for giving to gnomes.
+ **Teleporter** moves you at random to some other room.
+ **Torch** allows you to see in a dark room - tells you what’s in a dark room. 
 
**Moves**
+ **North** or **South** allows you to move up or down by one space.
+ **East** or **West** allows you to move right or left by one space.
+ **Teleport** allows you to teleport to some other room if you have a teleporter. 
+ Each move consumes 1 food unit and teleporting consumes 5. 

A project skeleton of the game has already been created for you in the src directory. Open main.rs to see how the game works and begin filling in the various TODOs throughout the rest of the .rc files. As you add more of your code to the project use Cargo to rebuild the game and ensure that it still compiles. Run the game from the command line to try out new features as you implement them. For how to get started with Cargo see [section 2](http://doc.rust-lang.org/book/getting-started.html) of *The Rust Programming Language* book.
