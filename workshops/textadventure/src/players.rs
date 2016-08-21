use board::Board;
use std::collections::VecDeque;
use std::io;

struct ExplorerData { x: u32, y: u32, energy: i32 }
struct GnomeData { x: u32, y: u32 }
struct LeprechaunData { x: u32, y: u32 }

enum Player {
    Explorer(ExplorerData),  // actual player
    Gnome(GnomeData),  // NPC
    Leprechaun(LeprechaunData),  // NPC
}

pub type Players = VecDeque<Player>;

pub fn build_players() -> Players {
    let mut players: Players = VecDeque::new();

    players.push_back(Player::Explorer(ExplorerData { x: 0, y: 0, energy: 99 }));
    players.push_back(Player::Gnome(GnomeData { x: 0, y: 4 }));
    players.push_back(Player::Leprechaun(LeprechaunData { x: 4, y: 4 }));

    players
}

pub fn take_turn(players: &mut Players, board: &Board) {
    let _player: Player;

    match players.pop_front() {
        Some(player) => match player {
            Player::Explorer(data) => {
                let _data = play_explorer(data, players, board);
                _player = Player::Explorer(_data);
            },
            Player::Gnome(data) => {
                let _data = play_gnome(data, players, board);
                _player = Player::Gnome(_data);
            },
            Player::Leprechaun(data) => {
                let _data = play_leprechaun(data, players, board);
                _player = Player::Leprechaun(_data);
            },
        },
        None => return,
    }

    players.push_back(_player)
}

fn play_explorer(data: ExplorerData, others: &Players, board: &Board) -> ExplorerData {
    ExplorerData { x: data.x, y: data.y, energy: data.energy }
}

fn play_gnome(data: GnomeData, others: &Players, board: &Board) -> GnomeData {
    GnomeData { x: data.x, y: data.y }
}

fn play_leprechaun(data: LeprechaunData, others: &Players, board: &Board) -> LeprechaunData {
    let mut _data = data;
    teleport_leprechaun(&mut _data, board);
    _data
}

fn teleport_explorer(data: &mut ExplorerData, board: &Board) -> bool {
    false
}

fn teleport_leprechaun(data: &mut LeprechaunData, board: &Board) {
    data.x = 4;
    data.y = 0;
}
