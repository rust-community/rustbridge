use players;
use inventory;
use std::io;

// TODO Need 5 by 5 not 2 by 1 game board.
pub type Board = [[Room; 1]; 2];

enum Wall { Opening, Solid, Magical }

struct Room {
    up: Wall,
    right: Wall,
    down: Wall,
    left: Wall,
    contents: Vec<inventory::Thing>
}

// TODO Construct a maze.
pub fn build_board() -> Board {
    [[Room {up: Wall::Opening, right: Wall::Solid,   down: Wall::Opening, left: Wall::Solid, contents: vec![]}],
     [Room {up: Wall::Opening, right: Wall::Opening, down: Wall::Solid,   left: Wall::Solid, contents: vec![]}]]
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

pub fn survey_room(player: players::Player, board: &Board) -> players::Player {
    // TODO
    unimplemented!()
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

fn xy_in_bounds(x: &i32, y: &i32, board: &Board) -> bool {
    x_in_bounds(x, board) && y_in_bounds(y, board)
}

fn x_in_bounds(x: &i32, board: &Board) -> bool {
    *x < board.len() as i32
}

fn y_in_bounds(y: &i32, board: &Board) -> bool {
    *y < board[0].len() as i32
}
