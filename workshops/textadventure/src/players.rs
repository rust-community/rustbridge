use board::Board;
use std::collections::VecDeque;
use std::io;

pub struct ExplorerData {
    pub x: u32, pub y: u32,
    pub energy: i32
}

pub struct GnomeData {
    pub x: u32, pub y: u32,
    pub energy: i32
}

pub struct LeprechaunData {
    pub x: u32, pub y: u32,
    pub energy: i32
}

pub enum Player {
    Explorer(ExplorerData),  // actual player
    Gnome(GnomeData),  // NPC
    Leprechaun(LeprechaunData),  // NPC
}

pub type Players = VecDeque<Player>;

pub fn build_players() -> Players {
    let mut players: Players = VecDeque::new();

    let explorer = Player::Explorer(ExplorerData {
        x: 0, y: 0,
        energy: 99
    });

    let gnome = Player::Gnome(GnomeData {
        x: 0, y: 4,
        energy: 111
    });

    let leprechaun = Player::Leprechaun(LeprechaunData {
        x: 4, y: 4,
        energy: 333
    });

    players.push_back(explorer);
    players.push_back(gnome);
    players.push_back(leprechaun);

    players
}

pub fn build_explorer_data(x: u32, y: u32, energy: i32) -> ExplorerData {
    ExplorerData{ x: x, y: y, energy: energy }
}

pub fn build_gnome_data(x: u32, y: u32, energy: i32) -> GnomeData {
    GnomeData{ x: x, y: y, energy: energy }
}

pub fn build_leprechaun_data(x: u32, y: u32, energy: i32) -> LeprechaunData {
    LeprechaunData{ x: x, y: y, energy: energy }
}

pub fn move_player(player: Player, board: &Board) -> Player {
    let _player : Player;

    match player {
        Player::Explorer(data) => {
            _player = Player::Explorer(move_explorer(data, board));
        },
        Player::Gnome(data) => {
            _player = Player::Gnome(move_gnome(data, board));
        },
        Player::Leprechaun(data) => {
            _player = Player::Leprechaun(move_leprechaun(data, board));
        }
    }

    _player
}

fn move_explorer(data: ExplorerData, board: &Board) -> ExplorerData {
    let mut _data = data;
    let mut input = String::new();

    loop {
        println!("Enter letter command: [U]p [R]ight [D]own [L]eft [T]eleport");

        match io::stdin().read_line(&mut input) {
            Ok(n) => (),
            Err(why) => { println!("Failed to read line: {:?}", why); continue; }
        }

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

fn move_explorer_up(data: &mut ExplorerData, board: &Board) {
    unimplemented!();
}

fn move_explorer_right(data: &mut ExplorerData, board: &Board) {
    unimplemented!();
}

fn move_explorer_down(data: &mut ExplorerData, board: &Board) {
    unimplemented!();
}

fn move_explorer_left(data: &mut ExplorerData, board: &Board) {
    unimplemented!();
}

fn teleport_explorer(data: &ExplorerData, board: &Board) -> bool {
    unimplemented!();    
    false
}

fn move_gnome(data: GnomeData, board: &Board) -> GnomeData {
    unimplemented!();
    GnomeData{ x: data.x, y: data.y, energy: data.energy }
}

fn move_leprechaun(data: LeprechaunData, board: &Board) -> LeprechaunData {
    unimplemented!();
    LeprechaunData{ x: data.x, y: data.y, energy: data.energy }
}   
