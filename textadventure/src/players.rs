extern crate rand;

use board;
use board::Board;
use board::Position;
use inventory;
use inventory::Thing;
use std::io;
use std::collections::VecDeque;
use self::rand::Rng;

pub type Players = VecDeque<Player>;

pub enum Player {
    Explorer(ExplorerData),  // user
    Gnome(GnomeData),  // NPC
    Leprechaun(LeprechaunData)  // NPC
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Direction { North, South, East, West }

pub struct ExplorerData {
    pos: Position,
    energy: i32,
    pub things: Vec<Thing>
}

pub struct GnomeData {
    pos: Position,
    energy: i32,
    pub things: Vec<Thing>
}

pub struct LeprechaunData {
    pos: Position,
    pub things: Vec<Thing>
}

pub fn build_players(board: &Board) -> Players {
    use self::Thing::*;

    let mut players: Players = VecDeque::new();

    let explorer = Player::Explorer(
        ExplorerData { pos: Position::new(0, 0, board),
                       energy: 65,
                       things: vec![Torch,
                                    GoldCoin { denom: 5 },
                                    GoldCoin { denom: 10 },
                                    GoldCoin { denom: 25 }] }
    );

    let gnome_a = Player::Gnome(
        GnomeData { pos: Position::new(0, 4, board),
                    energy: 41,
                    things: vec![GoldCoin { denom: 25 }; 3] }
    );

    let gnome_b = Player::Gnome(
        GnomeData { pos: Position::new(2, 2, board),
                    energy: 37,
                    things: vec![GoldCoin { denom: 25 }; 3] }
    );

    let mut lep_things = vec![];

    lep_things.append(&mut vec![GoldCoin { denom: 5 }; 3]);
    lep_things.append(&mut vec![GoldCoin { denom: 10 }; 3]);
    lep_things.append(&mut vec![GoldCoin { denom: 25 }; 6]);
    lep_things.append(&mut vec![FakeCoin { denom: 5 }; 2]);
    lep_things.append(&mut vec![FakeCoin { denom: 10 }; 2]);
    lep_things.append(&mut vec![FakeCoin { denom: 25 }; 5]);
    lep_things.append(&mut inventory::all_magic_words(board));
    lep_things.append(&mut inventory::all_fake_words(board));
    rand::thread_rng().shuffle(lep_things.as_mut_slice());

    let leprechaun = Player::Leprechaun(
        LeprechaunData { pos: Position::new(4, 4, board),
                         things: lep_things }
    );

    players.push_back(explorer);
    players.push_back(gnome_a);
    players.push_back(gnome_b);
    players.push_back(leprechaun);

    players
}

pub fn is_game_over(players: &Players) -> bool {
    players.iter()
           .filter(|&player| is_explorer(player))
           .fold(true, |acc, explorer| acc & is_dead(explorer))
}

pub fn move_player(player: Player, board: &Board) -> Player {
    let _player : Player;

    match player {
        Player::Explorer(data) => {
            _player = Player::Explorer(move_exp(data, board))
        },
        Player::Gnome(data) => {
            _player = Player::Gnome(move_gnome(data, board))
        },
        Player::Leprechaun(data) => {
            _player = Player::Leprechaun(move_lep(data, board))
        }
    }

    _player
}

pub fn is_occupant(other: &Player, pos: &Position) -> bool {
    match *other {
        Player::Explorer(ref data) => data.pos == *pos,
        Player::Gnome(ref data) => data.pos == *pos,
        Player::Leprechaun(ref data) => data.pos == *pos
    }
}

pub fn get_exp_pos(data: &ExplorerData) -> Position {
    data.pos
}

pub fn get_gnome_pos(data: &GnomeData) -> Position {
    data.pos
}

pub fn get_lep_pos(data: &LeprechaunData) -> Position {
    data.pos
}

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

fn move_exp(data: ExplorerData, board: &Board) -> ExplorerData {
    let mut _data = data;
    let mut input = String::new();

    loop {
        println!("Enter letter command: Move [N]orth [S]outh [E]ast [W]est or [T]eleport");

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(why) => {
                println!("Failed to read line: {:?}", why);
                input.clear();
                continue
            }
        }

        match input.trim().to_uppercase().chars().nth(0) {
            Some(command) => {
                match command {
                    'N' => { move_exp_north(&mut _data, board); break },
                    'S' => { move_exp_south(&mut _data, board); break },
                    'E' => { move_exp_east(&mut _data, board); break },
                    'W' => { move_exp_west(&mut _data, board); break },
                    'T' => if teleport_exp(&mut _data, board) { break }
                           else { println!("Cannot teleport") },
                    _ => println!("Invalid command")
                }
            },
            None => println!("Ignoring leading whitespace")
        }

        input.clear()
    }

    _data
}

fn dir_to_dx_dy(direction: &Direction) -> (i32, i32) {
    use self::Direction::*;

    match *direction {
        North => (0, 1),
        South => (0, -1),
        East => (1, 0),
        West => (-1, 0)
    }
}

fn move_gnome(data: GnomeData, board: &Board) -> GnomeData {
    use self::Direction::*;

    let pos = data.pos;
    let choices = [North, South, East, West];
    let (dx, dy) : (i32, i32);

    loop {
        let index = rand::thread_rng().gen_range(0, choices.len());
        let (_dx, _dy) = dir_to_dx_dy(&choices[index]);

        if board::move_in_bounds(&pos, &_dx, &_dy, board) {
            dx = _dx;
            dy = _dy;
            break
        }
    }

    GnomeData { pos: board::move_pos(pos, dx, dy, board),
                energy: data.energy - 1,
                things: data.things }
}

fn move_lep(data: LeprechaunData, board: &Board) -> LeprechaunData {
    let mut _data = data;

    teleport_lep(&mut _data, board);

    _data
}   

// TODO
fn teleport_lep(data: &mut LeprechaunData, board: &Board) {
}

// TODO
fn teleport_exp(data: &mut ExplorerData, board: &Board) -> bool {
    false
}

// TODO
fn move_exp_north(data: &mut ExplorerData, board: &Board) {
}

// TODO
fn move_exp_south(data: &mut ExplorerData, board: &Board) {
}

// TODO
fn move_exp_east(data: &mut ExplorerData, board: &Board) {
}

// TODO
fn move_exp_west(data: &mut ExplorerData, board: &Board) {
}

// TODO
fn has_word(word: &String, things: &Vec<Thing>) -> bool {
    false
}
