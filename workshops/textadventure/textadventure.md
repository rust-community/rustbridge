# Text Adventure

For this workshop, let’s build a simple text adventure game in Rust.

Here is what we need:

**Game Board** - A 5 by 5 grid containing a total of 25 rooms. 

**Rooms** - A **Room** may have up to 4 walls. Each **Wall** may be: **Open** which will allow unrestricted passage through; **Solid** which will prevent passage; or **Magical** which will allow passage only with a secret word. Some rooms are dark and some are light. 
 
**Characters** - We will have 3 kinds of characters. You the adventurous **Explorer**, wandering from room to room. **Leprechauns** the good wee people with a mischievous sense of humour. They can give you gold or magic words but you never know if they are telling the truth. They can teleport to any room. **Gnomes** who guard the treasures of the earth and just might take your stuff if you’re not careful. They can go one space in any direction - they know the magic word to walk through any kind of wall. 
 
**Things** - **Food** gives you **Energy** to keep going. **Gold** pays the way. **Pyrite** is given to you by mischievous leprechauns - also good for giving to gnomes. A **Teleporter** moves you at random to some other room. A **Torch** allows you to see in a dark room - tells you what’s in a dark room. 
 
**Moves** - **Up** or **Down** allows you to move up or down by one space. **Left** or **Right** allows you to move left or right by one space. **Teleport** allows you to teleport to some other room if you have a teleporter. 
 
Each move consumes 1 food unit and teleporting consumes 5. 

A project skeleton of the game has already been created for you in the src directory. Open main.rs to see how the game works and begin filling in the various TODOs throughout the rest of the .rc files. As you add more of your code to the project use Cargo to rebuild the game and ensure that it still compiles. Run the game from the command line to try out new features as you implement them. For how to get started with Cargo see [section 2](http://doc.rust-lang.org/book/getting-started.html) of *The Rust Programming Language* book.
