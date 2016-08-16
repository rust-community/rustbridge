pub enum Wall {
    Opening,
    Solid,
    Magical,
}

pub struct Room {
    up: Wall,
    right: Wall,
    down: Wall,
    left: Wall,
}

// TODO Need 5 by 5 not 2 by 1 game board.
pub type Board = [[Room; 1]; 2];

// TODO Construct a maze.
pub fn build_board() -> Board {
    [[Room {up: Wall::Opening, right: Wall::Solid,   down: Wall::Opening, left: Wall::Solid}],
     [Room {up: Wall::Opening, right: Wall::Opening, down: Wall::Solid,   left: Wall::Solid}]]
}
