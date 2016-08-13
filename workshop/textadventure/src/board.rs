pub enum Side {
    Opening,
    Solid,
    Magic,
}

pub struct Room {
    pub up: Side,
    pub right: Side,
    pub down: Side,
    pub left: Side,
}
