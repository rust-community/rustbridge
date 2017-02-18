extern crate rand;

use board;
use inventory;
use std::io;
use std::collections::VecDeque;
use self::rand::Rng;

pub type Players = VecDeque<Player>;

pub enum Player {
    Explorer(ExplorerData),  // actual player
    Gnome(GnomeData),  // NPC
    Leprechaun(LeprechaunData)  // NPC
}

pub struct ExplorerData {
    pub x: i32, pub y: i32,
    pub energy: i32,
    pub gold: i32, pub pyrite: i32,
    pub things: Vec<inventory::Thing>
}

pub struct GnomeData {
    pub x: i32, pub y: i32,
    pub things: Vec<inventory::Thing>
}

pub struct LeprechaunData {
    pub x: i32, pub y: i32
}

pub fn build_players() -> Players {
    let mut players: Players = VecDeque::new();

    let explorer = Player::Explorer(
        ExplorerData { x: 0, y: 0,
                       energy: 99,
                       gold: 7, pyrite: 5,
                       things: vec![inventory::Thing::Torch] }
    );

    let gnome = Player::Gnome(
        GnomeData { x: 0, y: 4,
                    things: vec![] }
    );

    let leprechaun = Player::Leprechaun(
        LeprechaunData { x: 4, y: 4 }
    );

    players.push_back(explorer);
    players.push_back(gnome);
    players.push_back(leprechaun);

    players
}

pub fn is_explorer_dead(players: &Players) -> bool {
    players.iter()
           .filter(|&p| is_explorer(p))
           .fold(false, |acc, e| acc & is_dead(e))
}

pub fn move_player(player: Player, board: &board::Board) -> Player {
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

enum Direction { North, South, East, West }

fn is_explorer(player: &Player) -> bool {
    match *player {
        Player::Explorer(_) => true,
        _ => false
    }
}

fn is_dead(player: &Player) -> bool {
    match *player {
        Player::Explorer(ref data) => data.energy <= 0,
        _ => false
    }
}

fn move_explorer(data: ExplorerData, board: &board::Board) -> ExplorerData {
    let mut _data = data;
    let mut input = String::new();

    loop {
        println!("Enter letter command: [N]orth [S]outh [E]ast [W]est [T]eleport");

        match io::stdin().read_line(&mut input) {
            Ok(n) => (),
            Err(why) => { println!("Failed to read line: {:?}", why); continue; }
        }

        match input.trim().chars().nth(0) {
            Some(command) => {
                match command {
                    'N' => { move_explorer_north(&mut _data, board); break; },
                    'S' => { move_explorer_south(&mut _data, board); break; },
                    'E' => { move_explorer_east(&mut _data, board); break; },
                    'W' => { move_explorer_west(&mut _data, board); break; },
                    'T' => if teleport_explorer(&mut _data, board) { break; }
                           else { println!("Cannot teleport"); },
                    _ => println!("Invalid command")
                }
            },
            None => println!("Ignoring leading whitespace")
        }
    }

    _data
}

fn direction_to_dx_dy(direction: &Direction) -> (i32, i32) {
    match *direction {
        Direction::North => (0, 1),
        Direction::South => (0, -1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0)
    }
}

fn move_gnome(data: GnomeData, board: &board::Board) -> GnomeData {
    let choices = [Direction::North, Direction::South, Direction::East, Direction::West];
    let (dx, dy) : (i32, i32);

    loop {
        let index = rand::thread_rng().gen_range(0, 4);
        let (_dx, _dy) = direction_to_dx_dy(&choices[index]);
        let xinb = board::x_in_bounds(data.x + _dx, board);
        let yinb = board::y_in_bounds(data.y + _dy, board);

        if xinb & yinb {
            dx = _dx;
            dy = _dy;
            break;
        }
    }

    GnomeData { x: data.x + dx, y: data.y + dy, things: data.things }
}

fn move_explorer_north(data: &mut ExplorerData, board: &board::Board) {
    unimplemented!();
}

fn move_explorer_south(data: &mut ExplorerData, board: &board::Board) {
    unimplemented!();
}

fn move_explorer_east(data: &mut ExplorerData, board: &board::Board) {
    unimplemented!();
}

fn move_explorer_west(data: &mut ExplorerData, board: &board::Board) {
    unimplemented!();
}

fn teleport_explorer(data: &ExplorerData, board: &board::Board) -> bool {
    unimplemented!();
}

fn move_leprechaun(data: LeprechaunData, board: &board::Board) -> LeprechaunData {
    unimplemented!();
}   
