use board::Board;
use std::io;

pub trait Player {
    fn take_turn(&self, board: &Board, players: &Players) -> bool;
}

pub type Players = Vec<Box<Player>>;

trait Movable {
    fn move_up(&self, &Board) -> bool;
    fn move_right(&self, &Board) -> bool;
    fn move_down(&self, &Board) -> bool;
    fn move_left(&self, &Board) -> bool;
}

trait Teleportable {
    fn teleport(&self, &Board) {
        // TODO Jump to any other space on the game board at random.
        unimplemented!(); 
    }
}

pub struct Explorer { x: u32, y: u32, energy: i32 }  // actual player
struct Gnome { x: u32, y: u32 }  // NPC
struct Leprechaun { x: u32, y: u32 }  // NPC

impl Player for Explorer {
    fn take_turn(&self, board: &Board, players: &Players) -> bool {
        let mut input = String::new();
        
        io::stdin().read_line(&mut input)
                .expect("Failed to read line");

        // TODO Parse input and move or teleport Explorer accordingly.
        // If Explorer has a Teleporter and chooses to use it then teleport.
        // Otherwise attempt to move Explorer one space left, right, up or down.
        unimplemented!();

        true  // TODO Return false when Explorer runs out of energy.
     }
}

impl Movable for Explorer {
    // TODO Attempt to Move Explorer one space away from current x, y position.
    // A move will fail if an Explorer attempts to pass through a Wall or 
    // travel outside the boundaries of the Board.
    fn move_up(&self, board: &Board) -> bool {unimplemented!(); false }
    fn move_right(&self, board: &Board) -> bool { unimplemented!(); false }
    fn move_down(&self, board: &Board) -> bool { unimplemented!(); false }
    fn move_left(&self, board: &Board) -> bool { unimplemented!(); false }
}

impl Teleportable for Explorer {}

impl Player for Gnome {
    fn take_turn(&self, board: &Board, players: &Players) -> bool {
        // TODO Move Gnome one space left, right, up or down.
        // Do not waste a turn attempting to move outside the Board.
        unimplemented!();

        true  // Gnomes do not die.
    }
}

impl Movable for Gnome {
    // TODO Move Gnome one space away from current x, y position.
    // Gnomes can pass through any Wall: Opening, Magical or Solid.
    // Gnomes cannot leave the confines of the board.
    fn move_up(&self, board: &Board) -> bool { unimplemented!(); false }
    fn move_right(&self, board: &Board) -> bool { unimplemented!(); false }
    fn move_down(&self, board: &Board) -> bool { unimplemented!(); false }
    fn move_left(&self, board: &Board) -> bool { unimplemented!(); false }
}

impl Player for Leprechaun {
    fn take_turn(&self, board: &Board, players: &Players) -> bool {
        // TODO Teleport Leprechaun to any space on the Board at random.
        unimplemented!();

        true  // Leprechauns do not die.
    }
}

impl Teleportable for Leprechaun {}

pub fn build_players() -> Players {
    let mut players: Players = Vec::new();

    players.push(Box::new(Explorer { x: 0, y: 0, energy: 99 }));
    players.push(Box::new(Gnome { x: 0, y: 4 }));
    players.push(Box::new(Leprechaun { x: 4, y: 4 }));

    players
}
