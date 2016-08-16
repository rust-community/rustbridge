mod board;
mod players;

use board::build_board;
use players::build_players;

fn main() {
    println!("Welcome to the maze! Let's begin.");

    let board = build_board();
    let players = build_players();

    loop {
        for player in &players {
            if !player.take_turn(&board, &players) {
                println!("You have died!");
                break;
            }
        }
    }

    println!("GAME OVER");
}
