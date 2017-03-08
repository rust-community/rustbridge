pub mod board;
pub mod players;
pub mod inventory;

use board::display_map;

fn main() {
    println!("Welcome to the maze; Let's begin");

    let mut board = board::build_board();
    let mut players = players::build_players(&board);

    loop {
        display_map(&board, &players);
        match players.pop_front() {
            Some(player) => {
                if players::is_game_over(&players) { break };
                let ready = board::scavenge(player, &mut board);
                let moved = players::move_player(ready, &board);
                let played = inventory::encounter_others(moved, &mut players);
                players.push_back(played);
            }
            None => break
        }
    }

    println!("GAME OVER");
}
