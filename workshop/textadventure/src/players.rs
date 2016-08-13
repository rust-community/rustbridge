use std::io;

// TODO methods need game board arg
trait Movable {
    fn move_up(&self) -> bool;
    fn move_right(&self) -> bool;
    fn move_down(&self) -> bool;
    fn move_left(&self) -> bool;
}

pub struct Player {}

impl Player {
    pub fn take_turn(&self) -> bool {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // TODO parse and process input

        return true;
    }
}
