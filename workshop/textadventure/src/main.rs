mod board;

use board::Room;
use board::Side;
use std::io;

struct Player {}

impl Player {
    fn take_turn(&self) -> bool {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // TODO parse and process input

        return true;
    }
}

fn main() {
    println!("Welcome to Gnomes! Let's begin.");

    //let board = [[Room; 5]; 5];  // 5 by 5 game board
    let room = Room {up: Side::Solid, right: Side::Opening, down: Side::Solid, left: Side::Magic};
    let player = Player {};  // user
    let gnome = Player {};  // NPC
    let leprechaun = Player {};  // NPC

    loop {
        if !player.take_turn() {  // not alive
            break;
        }

        gnome.take_turn();
        leprechaun.take_turn();
    }

    println!("You have died!  GAME OVER");
}
