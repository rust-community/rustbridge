pub enum Side {
    Opening,
    Solid,
    Magic,
}

pub struct Room {
    up: Side,
    right: Side,
    down: Side,
    left: Side,
}

// TODO Need 5 by 5 not 2 by 1 game board
pub fn build_board() -> [[Room; 1]; 2] {
    [[Room {up: Side::Solid,   right: Side::Solid,   down: Side::Opening, left: Side::Solid}],
     [Room {up: Side::Opening, right: Side::Opening, down: Side::Solid,   left: Side::Solid}]]
}
