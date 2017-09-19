# Text Adventure

[Code](https://github.com/rust-community/rustbridge/tree/master/workshops/src/textadventure/src)

For this workshop, let’s add various features to a simple text adventure game written in Rust.

Here's what we have to work with:

**Game Board** - A 5 by 5 grid containing a total of 25 rooms. 

**Rooms** - A room may have up to 4 **Walls**. Each wall may be:
+ **Open** which will allow unrestricted passage through.
+ **Solid** which will prevent passage.
+ **Magical** which will allow passage only with a secret word.
+ The walls can be configured to form a maze.
+ A room can be light or dark depending on whether or not it contains a torch.
 
**Characters** - We will have 3 kinds of characters.
+ You the adventurous **Explorer**, wandering from room to room.
+ **Leprechauns** the good wee people with a mischievous sense of humor. They can give you gold or **Magic Words** but you never know if they are telling the truth. They can teleport to any room.
+ **Gnomes** who guard the treasures of the earth and just might take your stuff if you’re not careful. They can go one space in any direction - they know the magic word to walk through any kind of wall. 
 
**Things** - Things can be picked up and exchanged by characters.
+ **Food** gives you **Energy** to keep going.
+ **Gold** pays the way.
+ **Fake Gold** is given to you by mischievous leprechauns - also good for giving to gnomes.
+ **Teleporter** moves you at random to some other room.
+ **Torch** allows you to see in a dark room - tells you what’s in a dark room. 
 
**Moves**
+ **North** or **South** allows you to move up or down by one space.
+ **East** or **West** allows you to move right or left by one space.
+ **Teleport** allows you to teleport to some other room if you have a teleporter. 
+ Each move consumes 1 food unit and teleporting consumes 5. 

Much of the game has already been implemented for you but is not very playable.  Your job is to implement the remaining **TODO**s so that the game plays according to the rules listed above.  All the Rust code for the game is in the **src** directory.  The code already compiles and runs although there are numerous compiler warnings.  These warnings should disappear as you build out the rest of the game.  Look at the game loop in **main.rs** to get an idea of how the game is structured then begin adding your missing code to the **players**, **board** and **inventory** modules.

As you add more of your code to your game use Cargo to check it and ensure that it still compiles. Run the game from the command line to try out new features as you implement them. For how to get started with Rust and its Cargo tool see [Chapter 2](http://rust-lang.github.io/book/second-edition/ch02-00-guessing-game-tutorial.html) of [*The Rust Programming Language*](http://rust-lang.github.io/book/second-edition) book.  Other good resources for learning Rust syntax and semantics are [*Rust by Example*](http://rustbyexample.com) and the [*Rustlings*](https://github.com/carols10cents/rustlings) project.

If you forked this repo on GitHub you can check the following tasks off as you complete them by editing this **README** as you make your commmits.  This **README.md** markdown file is in the **textadventure** directory above the **src** directory.  Here is the order I recommend implementing the remaining features:

- [ ] First implement the `is_opening` function in **board.rs**.  You should use the `pos_to_room` function that is already provided for you to get at the wall in question.

- [ ] Next implement the `move_exp_direction` function in **players.rs** so the explorer can move.  See `move_gnome` for the mechanics of how this function should work.  Keep in mind that unlike `move_gnome` the direction of an explorer's move is chosen interactively by the user and an explorer can only walk through openings or magical walls.  I wouldn't worry about magical walls or secret words yet.  Just treat any magical walls as solid for now.  Don't forget that moving one space or walking into a wall consume 1 unit of an explorer's energy per turn.

- [ ] Now implement the `teleport_lep` and `teleport_exp` functions in **players.rs**.  See `move_gnome` for how to move an NPC at random.  Unlike gnomes which always move only one space every turn teleporting can put a player anywhere on the board.  The `teleport_exp` function returns a **bool** because an explorer can only teleport if she has a teleporter.  See the `build_players` function to get an idea of what an explorer's things are.  Don't forget that teleporting consumes 5 units of an explorer's energy.  Leprechauns are tireless so they don't lose any energy.

- [ ] Next implement the `exp_has_teleporter` function in **inventory.rs**.  Take a look at the [any method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) of Rust's Iterator trait.

- [ ] Now implement the `exp_has_torch` function in **inventory.rs** and the `room_has_torch` function in **board.rs**.  Once that is done the explorer can see the contents of rooms since she is already carrying a torch.  Similarly, if an explorer has no torch but enters a room containing one she can pick up that torch because the room is already lit by it.

- [ ] Next implement the `exp_pick_up_food`, `exp_pick_up_coins`, `exp_pick_up_teleporter` and `exp_pick_up_torch` functions in **board.rs**.  These functions remove things from a room's contents and add it to the explorer's things.  Use the `get_exp_pos`, `pos_to_room` and `pick_up_thing` functions that are already provided for you.  Don't forget to assign the new room returned by `pick_up_thing` back to its position in the `board` array otherwise the thing[s] being picked up will remain in the room.  An explorer does not put down the things she picks up but she can use coins to pay gnomes for passage or have all her things stolen if she cannot or refuses to pay.

- [ ] Now implement the `exp_eat_food` function in **players.rs**.  This function removes all food items from an explorer's things and adds their energy values to the explorer's energy.  An explorer eats any food as soon as she picks it up instead of carrying it around.  Food is destroyed by eating it and is not replenished.  Otherwise the game would never end.

- [ ] Now implement the `shake_down` function in **inventory.rs**.  The **inventory** module defines the encounters or exchanges between players.  These exchanges happen when players are occupants of the same room.  A gnome will demand gold coins from an explorer for safe passage through a room.  If the explorer cannot or chooses not to pay the gnome will take all of the explorer's things.  Hence the name of the function.  An explorer can pay a gnome with fake coins but that only works once.  That same gnome will always rob the explorer of everything on the next encounter.  This function should be highly interactive and decomposed into smaller functions.  See `exp_scavenge` in **board.rs** for ideas.

- [ ] Next implement the `trick_or_treat` function in **inventory.rs**.  A leprechaun will give an explorer coins or secret words when encountering her.  Concentrate on just coins for now.  A leprechaun starts out with a finite number of gold and fake coins of various denominations.  Those coins are shuffled at creation time so a leprechaun should be able to hand them out one-at-a-time in whatever order they're in.  

- [ ] Now implement the `gnome_scavenge` function in **board.rs**.  This function makes the game more challenging by enabling gnomes to compete with the explorer for items scattered across the board.  A gnome now picks up all the same things that an explorer does in `exp_scavenge` but it does so automatically rather than interactively because it is an NPC.  Like `exp_scavenge`, `gnome_scavenge` can be decomposed into several smaller functions if need be.  The names and signatures of these smaller functions have not been defined for you.  If you really want to get fancy you can have gnomes stockpile gold and other non-perishable items in certain rooms so that an explorer can pick them up again.

- [ ] Next implement the `all_magic_words` and `all_fake_words` functions and finish the `trick_or_treat` function in **inventory.rs**.  The first 2 functions supply the leprechaun with magic and fake secret words to give to the explorer.  The list of all magic words can be generated by iterating through all the walls in the game board and filtering out all the walls that aren't magical.  The secret words can then be extracted from the remaining magical walls.  The list of all fake words can be whatever you want as long as it excludes the real magic words from the board.  Make sure it is apparent what secret word is being given to an explorer when `trick_or_treat` executes.

- [ ] Lastly implement the means for an explorer to walk through magical walls.  There are 3 magical walls placed strategically throughout the board for you to test this feature out on.  While an explorer has no knowledge of what walls are solid or magical you can spot which walls are magical by looking closely at the ASCII rendition of the board. Modify the `move_exp_direction` function in **players.rs** so that if an explorer walks into a wall the user is queried for a secret word that she must type out in its entirety.  Implement the `has_word` function to search a collection of things for said word.  If said word is found among the explorer's things then call the `open_sesame` function in **board.rs** with that word.  In addition to a secret word string, `open_sesame` also takes an explorer's source and target positions on the board.  If the wall an explorer is attempting to traverse is magical and the secret word matches then `open_sesame` returns **true** and the explorer is allowed to pass through the wall.

 The original rules for this game are from a homework assignment devised by [Dr. Jim Peckol](http://www.ee.washington.edu/people/jim-peckol) at the University of Washington.  This game was the final project for his introductory C++ class that emphasized UML and object-oriented design.  For this Rust workshop, I chose to implement the game in a more functional style using enums and pattern matching for dispatching.  Rust is a multi-paradigm language so the same results could have been achieved using traits and polymorphism.  See Lee Baillie's [*text-based adventure game*](https://github.com/tildeio/learning-rust) for an example of such an alternate approach.
