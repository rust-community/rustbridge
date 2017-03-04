extern crate rand;

use board;
use inventory;
use std::io;
use std::collections::VecDeque;
use self::rand::Rng;

pub type Players = VecDeque<Player>;

pub enum Player {
    Explorer(ExplorerData),  // user
    Gnome(GnomeData),  // NPC
    Leprechaun(LeprechaunData)  // NPC
}

pub struct ExplorerData {
    pos: board::Position,
    energy: i32,
    things: Vec<inventory::Thing>
}

pub struct GnomeData {
    pos: board::Position,
    things: Vec<inventory::Thing>
}

pub struct LeprechaunData {
    pos: board::Position,
    things: Vec<inventory::Thing>
}

pub fn build_players(board: &board::Board) -> Players {
    let mut players: Players = VecDeque::new();

    let explorer = Player::Explorer(
        ExplorerData { pos: board::Position::new(0, 0, board),
                       energy: 99,
                       things: vec![inventory::Thing::Torch,
                                    inventory::Thing::GoldCoin { denom: 5 },
                                    inventory::Thing::GoldCoin { denom: 10 },
                                    inventory::Thing::GoldCoin { denom: 25 }] }
    );

    let gnome = Player::Gnome(
        GnomeData { pos: board::Position::new(0, 4, board),
                    things: vec![inventory::Thing::GoldCoin { denom: 25 },
                                 inventory::Thing::GoldCoin { denom: 25 },
                                 inventory::Thing::GoldCoin { denom: 25 }] }
    );

    let mut pot_of_gold = vec![];

    pot_of_gold.append(&mut inventory::gold_coins());
    pot_of_gold.append(&mut inventory::fake_coins());
    pot_of_gold.append(&mut inventory::fake_words());

    let leprechaun = Player::Leprechaun(

        LeprechaunData { pos: board::Position::new(4, 4, board),
                         things: pot_of_gold }
    );

    players.push_back(explorer);
    players.push_back(gnome);
    players.push_back(leprechaun);

    players
}

pub fn is_game_over(players: &Players) -> bool {
    players.iter()
           .filter(|&player| is_explorer(player))
           .fold(true, |acc, explorer| acc & is_dead(explorer))
}

pub fn move_player(player: Player, board: &board::Board) -> Player {
    let _player : Player;

    match player {
        Player::Explorer(data) => {
            _player = Player::Explorer(move_exp(data, board));
        },
        Player::Gnome(data) => {
            _player = Player::Gnome(move_gnome(data, board));
        },
        Player::Leprechaun(data) => {
            _player = Player::Leprechaun(move_lep(data, board));
        }
    }

    _player
}

pub fn is_occupant(other: &Player, pos: &board::Position) -> bool {
    match *other {
        Player::Explorer(ref data) => data.pos == *pos,
        Player::Gnome(ref data) => data.pos == *pos,
        Player::Leprechaun(ref data) => data.pos == *pos
    }
}

pub fn get_exp_pos(data: &ExplorerData) -> board::Position {
    data.pos
}

pub fn get_gnome_pos(data: &GnomeData) -> board::Position {
    data.pos
}

pub fn get_lep_pos(data: &LeprechaunData) -> board::Position {
    data.pos
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

fn move_exp(data: ExplorerData, board: &board::Board) -> ExplorerData {
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
                    'N' => { move_exp_north(&mut _data, board); break; },
                    'S' => { move_exp_south(&mut _data, board); break; },
                    'E' => { move_exp_east(&mut _data, board); break; },
                    'W' => { move_exp_west(&mut _data, board); break; },
                    'T' => if teleport_exp(&mut _data, board) { break; }
                           else { println!("Cannot teleport"); },
                    _ => println!("Invalid command")
                }
            },
            None => println!("Ignoring leading whitespace")
        }
    }

    _data
}

fn dir_to_dx_dy(direction: &Direction) -> (i32, i32) {
    match *direction {
        Direction::North => (0, 1),
        Direction::South => (0, -1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0)
    }
}

fn move_gnome(data: GnomeData, board: &board::Board) -> GnomeData {
    let pos = data.pos;
    let choices = [Direction::North, Direction::South, Direction::East, Direction::West];
    let (dx, dy) : (i32, i32);

    loop {
        let index = rand::thread_rng().gen_range(0, choices.len());
        let (_dx, _dy) = dir_to_dx_dy(&choices[index]);

        if board::move_in_bounds(&pos, &_dx, &_dy, board) {
            dx = _dx;
            dy = _dy;
            break;
        }
    }

    GnomeData { pos: board::move_pos(pos, dx, dy, board),
                things: data.things }
}

fn move_lep(data: LeprechaunData, board: &board::Board) -> LeprechaunData {
    let mut _data = data;

    teleport_lep(&mut _data, board);

    _data
}   

fn teleport_lep(data: &mut LeprechaunData, board: &board::Board) {
    unimplemented!()
}

fn move_exp_north(data: &mut ExplorerData, board: &board::Board) {
    unimplemented!()
}

fn move_exp_south(data: &mut ExplorerData, board: &board::Board) {
    unimplemented!()
}

fn move_exp_east(data: &mut ExplorerData, board: &board::Board) {
    unimplemented!()
}

fn move_exp_west(data: &mut ExplorerData, board: &board::Board) {
    unimplemented!()
}

fn teleport_exp(data: &mut ExplorerData, board: &board::Board) -> bool {
    unimplemented!()
}
