pub mod board;
pub mod players;
pub mod inventory;

use board::build_board;
use players::build_players;
use players::move_player;
use inventory::encounter_player;

fn main() {
    println!("Welcome to the maze; Let's begin");

    let board = build_board();
    let mut players = build_players();

    loop {
        match players.pop_front() {
            Some(player) => {
                let moved = move_player(player, &board);
                let played = encounter_player(moved, &mut players);
                players.push_back(played);
            }
            None => break
        }
    }

    println!("GAME OVER");
}
