use board::Board;
use std::collections::VecDeque;
use std::io;

struct ExplorerData {
    x: u32, y: u32,
    gold: u32, pyrite: u32,
    energy: i32
}

struct GnomeData {
    x: u32, y: u32,
    gold: u32, pyrite: u32
}

struct LeprechaunData {
    x: u32, y: u32,
    gold: u32, pyrite: u32
}

enum Player {
    Explorer(ExplorerData),  // actual player
    Gnome(GnomeData),  // NPC
    Leprechaun(LeprechaunData),  // NPC
}

pub type Players = VecDeque<Player>;

pub fn build_players() -> Players {
    let mut players: Players = VecDeque::new();

    let explorer = Player::Explorer(ExplorerData {
        x: 0, y: 0,
        gold: 5, pyrite: 0,
        energy: 99
    });

    let gnome = Player::Gnome(GnomeData {
        x: 0, y: 4,
        gold: 11, pyrite: 3
    });

    let leprechaun = Player::Leprechaun(LeprechaunData {
        x: 4, y: 4,
        gold: 32, pyrite: 45
    });

    players.push_back(explorer);
    players.push_back(gnome);
    players.push_back(leprechaun);

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
            }
        },
        None => return
    }

    players.push_back(_player)
}

fn play_explorer(data: ExplorerData, others: &Players, board: &Board) -> ExplorerData {
    let mut _data = data;
    let mut input = String::new();

    loop {
        println!("Enter letter command: [U]p [R]ight [D]own [L]eft [T]eleport");

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().chars().nth(0) {
            Some(command) => {
                match command {
                    'U' => { move_explorer_up(&mut _data, board); break; },
                    'R' => { move_explorer_right(&mut _data, board); break; },
                    'D' => { move_explorer_down(&mut _data, board); break; },
                    'L' => { move_explorer_left(&mut _data, board); break; },
                    'T' => if teleport_explorer(&mut _data, board) { break; },
                    _ => println!("Invalid command")
                }
            },
            None => println!("Ignoring leading whitespace")
        }
    }

    _data
}

fn play_gnome(data: GnomeData, others: &Players, board: &Board) -> GnomeData {
    let mut _data = data;
    move_gnome(&mut _data, board);
    _data
}

fn play_leprechaun(data: LeprechaunData, others: &Players, board: &Board) -> LeprechaunData {
    let mut _data = data;
    teleport_leprechaun(&mut _data, board);
    _data
}

fn move_explorer_up(data: &mut ExplorerData, board: &Board) -> bool {
    unimplemented!();
    false
}

fn move_explorer_right(data: &mut ExplorerData, board: &Board) -> bool {
    unimplemented!();
    false
}

fn move_explorer_down(data: &mut ExplorerData, board: &Board) -> bool {
    unimplemented!();
    false
}

fn move_explorer_left(data: &mut ExplorerData, board: &Board) -> bool {
    unimplemented!();
    false
}

fn move_gnome(data: &mut GnomeData, board: &Board) {
    unimplemented!();
}

fn teleport_explorer(data: &mut ExplorerData, board: &Board) -> bool {
    unimplemented!();    
    false
}

fn teleport_leprechaun(data: &mut LeprechaunData, board: &Board) {
    unimplemented!();
}
