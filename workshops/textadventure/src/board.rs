extern crate serde_json;

use players;
use players::Player;
use players::Players;
use players::ExplorerData;
use players::GnomeData;
use inventory;
use inventory::Thing;
use std::io;

// FIXME Need 5 by 5 not 2 by 1 game board.
pub type Board = [[Room; 1]; 2];

enum Wall {
    Opening,
    Solid,
    Magical { word: String }
}

struct Room {
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
    contents: Vec<Thing>
}

// FIXME Load board from JSON file
pub fn build_board() -> Board {
    use self::Wall::*;

    [[Room {north: Opening, east: Solid,   south: Opening, west: Solid, contents: vec![]}],
     [Room {north: Opening, east: Opening, south: Solid,   west: Solid, contents: vec![]}]]
}

// TODO Iterate through Board
pub fn display_map(board: &Board, players: &Players) {
    unimplemented!()
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32
}

impl Position {
    pub fn new(x: i32, y: i32, board: &Board) -> Position {
        if !xy_in_bounds(&x, &y, board) {
            panic!("position out of bounds");
        }

        Position { x: x, y: y }
    }
}

pub fn move_in_bounds(pos: &Position, dx: &i32, dy: &i32, board: &Board) -> bool {
    let x = pos.x + dx;
    let y = pos.y + dy;

    xy_in_bounds(&x, &y, board)
}

pub fn move_pos(pos: Position, dx: i32, dy: i32, board: &Board) -> Position {
    if !move_in_bounds(&pos, &dx, &dy, board) {
        panic!("move out of bounds")
    }

    let x = pos.x + dx;
    let y = pos.y + dy;

    Position::new(x, y, board)
}

pub fn scavenge(player: Player, board: &mut Board) -> Player {
    let _player: Player;

    match player {
        Player::Explorer(exp) => {
            _player = Player::Explorer(exp_scavenge(exp, board));
        },
        Player::Gnome(gnome) => {
            _player = Player::Gnome(gnome_scavenge(gnome, board));
        },
        Player::Leprechaun(lep) => {
            _player = Player::Leprechaun(lep);
        }
    }

    _player
}

// FIXME
fn pos_to_room<'a, 'b>(pos: &'a Position, board: &'b Board) -> &'b Room {
    unimplemented!()
}

// FIXME ASCII graphics
fn draw_room(room: &Room, occupant: Option<&Player>) {
    unimplemented!()
}

// FIXME
fn display_room_contents(room: &Room) {
    unimplemented!()
}

fn xy_in_bounds(x: &i32, y: &i32, board: &Board) -> bool {
    x_in_bounds(x, board) && y_in_bounds(y, board)
}

fn x_in_bounds(x: &i32, board: &Board) -> bool {
    *x < board.len() as i32
}

fn y_in_bounds(y: &i32, board: &Board) -> bool {
    *y < board[0].len() as i32
}

fn exp_scavenge(data: ExplorerData, board: &mut Board) -> ExplorerData {
    let mut exp = data;
    let mut input = String::new();

    if room_has_torch(&players::get_exp_pos(&exp), board) || inventory::exp_has_torch(&exp) {
        // FIXME loop until room is empty or done
        loop {
            inventory::display_exp_things(&exp);
            display_room_contents(pos_to_room(&players::get_exp_pos(&exp), board));

            // FIXME display commands conditionally
            println!("Enter letter command: FIXME");
            
            match io::stdin().read_line(&mut input) {
                Ok(n) => (),
                Err(why) => { println!("Failed to read line: {:?}", why); continue; }
            }

            match input.trim().chars().nth(0) {
                Some(command) => {
                    match command {
                        'F' => { exp_pick_up_food(&mut exp, board); exp_eat_food(&mut exp) },
                        'C' => exp_pick_up_coins(&mut exp, board),
                        'X' => exp_pick_up_teleporter(&mut exp, board),
                        'T' => exp_pick_up_torch(&mut exp, board),
                        'D' => break,
                        _ => println!("Invalid command")
                    }
                },
                None => println!("Ignoring leading whitespace")
            }
        }
    } else {
        inventory::display_exp_things(&exp);
    }

    exp
}

// TODO
fn latest_occupant<'a, 'b, 'c>(pos: &Position, board: &Board, players: Players) -> Option<&'c Player> {
    None
}

// TODO
fn gnome_scavenge(data: GnomeData, board: &mut Board) -> GnomeData {
    let mut gnome = data;

    // Your code goes here.

    gnome
}

// TODO
fn room_has_food(pos: &Position, board: &Board) -> bool {
    false
}

// TODO
fn room_has_coins(pos: &Position, board: &Board) -> bool {
    false
}

// TODO
fn room_has_teleporter(pos: &Position, board: &Board) -> bool {
    false
}

// TODO
fn room_has_torch(pos: &Position, board: &Board) -> bool {
    false
}

// TODO
fn exp_pick_up_food(exp: &mut ExplorerData, board: &mut Board) {
    unimplemented!()
}

// TODO
fn exp_eat_food(exp: &mut ExplorerData) {
    unimplemented!()
}

// TODO
fn exp_pick_up_coins(exp: &mut ExplorerData, board: &mut Board) {
    unimplemented!()
}

// TODO
fn exp_pick_up_teleporter(exp: &mut ExplorerData, board: &mut Board) {
    unimplemented!()
}

// TODO
fn exp_pick_up_torch(exp: &mut ExplorerData, board: &mut Board) {
    unimplemented!()
}
