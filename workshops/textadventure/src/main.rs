mod board;
mod players;

use board::build_board;
use players::build_players;
use players::take_turn;

fn main() {
    println!("Welcome to the maze! Let's begin.");

    let board = build_board();
    let mut players = build_players();

    loop {
        take_turn(&mut players, &board);
    }

    println!("GAME OVER");
}
