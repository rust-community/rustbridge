mod board;
mod players;

use board::build_board;
use players::Player;

fn main() {
    println!("Welcome to Gnomes! Let's begin.");

    let board = build_board();
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
