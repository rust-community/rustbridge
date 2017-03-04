pub mod board;
pub mod players;
pub mod inventory;

fn main() {
    println!("Welcome to the maze; Let's begin");

    let board = board::build_board();
    let mut players = players::build_players(&board);

    loop {
        match players.pop_front() {
            Some(player) => {
                if players::is_game_over(&players) { break };
                let ready = board::survey_room(player, &board);
                let moved = players::move_player(ready, &board);
                let played = inventory::encounter_others(moved, &mut players);
                players.push_back(played);
            }
            None => break
        }
    }

    println!("GAME OVER");
}
