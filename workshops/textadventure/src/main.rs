pub mod board;
pub mod players;
pub mod inventory;

fn main() {
    println!("Welcome to the maze.");

    let mut board = board::build_board();
    let mut players = players::build_players(&board);

    loop {
        board::display_map(&board, &players);
        if players::is_game_over(&players) { break };
        match players.pop_front() {
            Some(player) => {
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
