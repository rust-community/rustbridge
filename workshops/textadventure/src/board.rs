use inventory;
use std::io;

pub enum Wall { Opening, Solid, Magical }

pub struct Room {
    up: Wall,
    right: Wall,
    down: Wall,
    left: Wall,
    contents: Vec<inventory::Thing>
}

// TODO Need 5 by 5 not 2 by 1 game board.
pub type Board = [[Room; 1]; 2];

// TODO Construct a maze.
pub fn build_board() -> Board {
    [[Room {up: Wall::Opening, right: Wall::Solid,   down: Wall::Opening, left: Wall::Solid, contents: vec![]}],
     [Room {up: Wall::Opening, right: Wall::Opening, down: Wall::Solid,   left: Wall::Solid, contents: vec![]}]]
}

pub fn x_in_bounds(x: i32, board: &Board) -> bool {
    x < board.len() as i32
}

pub fn y_in_bounds(y: i32, board: &Board) -> bool {
    y < board[0].len() as i32
}
