use players;
use players::Player;
use players::Players;
use players::ExplorerData;
use players::GnomeData;
use inventory;
use inventory::Thing;
use std::io;

// 5 by 5 room game board
pub type Board = [[Room; 5]; 5];

#[derive(Clone, Debug)]
pub struct Room {
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
    contents: Vec<Thing>
}

impl Room {
    fn pick_up_thing(&mut self, index: usize) -> Thing {
        self.contents.remove(index)
    }
}

#[derive(Clone, Debug)]
enum Wall {
    Solid,
    Opening,
    Magical { word: String }
}

// A hard-coded maze definition.  A maze generator would be a lot cooler.
// Since the game board is modeled as a 2D array for simplicity walls are
// double-sided so their types need to match up across adjacent rooms.
pub fn build_board() -> Board {
    use self::Wall::*;
    use inventory::Thing::*;

    [[Room {north: Solid, east: Solid, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Solid, west: Opening,
            contents: vec![GoldCoin { denom: 5 }, GoldCoin { denom: 10 }]},
      Room {north: Solid, east: Opening, south: Solid, west: Opening,
            contents: vec![]},
      Room {north: Solid, east: Solid, south: Opening, west: Opening,
            contents: vec![Food { name: String::from("chicken"), energy: 8 }]}
     ],
     [Room {north: Opening, east: Opening, south: Magical { word: String::from("aberto") }, west: Solid,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Solid, west: Opening,
            contents: vec![Torch, Food { name: String::from("ham"), energy: 9 }]},
      Room {north: Solid, east: Opening, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Opening, west: Opening,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Opening, west: Opening,
            contents: vec![GoldCoin { denom: 10 }, GoldCoin { denom: 25 }]}
     ],
     [Room {north: Magical { word: String::from("aberto") }, east: Opening, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Solid, south: Opening, west: Opening,
            contents: vec![GoldCoin { denom: 10 }, GoldCoin { denom: 10 }]},
      Room {north: Opening, east: Magical { word: String::from("godzilla") }, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Opening, west: Magical { word: String::from("godzilla") },
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Solid, west: Solid,
            contents: vec![Torch]}
     ],
     [Room {north: Opening, east: Solid, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Opening, east: Opening, south: Solid, west: Solid,
            contents: vec![Torch]},
      Room {north: Opening, east: Solid, south: Solid, west: Opening,
            contents: vec![Food { name: String::from("steak"), energy: 12 }]},
      Room {north: Opening, east: Opening, south: Solid, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Solid, south: Opening, west: Opening,
            contents: vec![]}
     ],
     [Room {north: Opening, east: Opening, south: Solid, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Magical { word: String::from("shazam") }, south: Solid, west: Opening,
            contents: vec![GoldCoin { denom: 25 }, GoldCoin { denom: 10 }, GoldCoin { denom: 5 }]},
      Room {north: Solid, east: Opening, south: Solid, west: Magical { word: String::from("shazam") },
            contents: vec![Teleporter, Food { name: String::from("fruit"), energy: 6 }]},
      Room {north: Solid, east: Opening, south: Solid, west: Opening,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Solid, west: Opening,
            contents: vec![GoldCoin { denom: 25 }, GoldCoin { denom: 25 }]}
     ]]
}

// Displays the game board and players' positions on it as ASCII art.
pub fn display_map(board: &Board, players: &Players) {
    for room in board[0].iter() {
        match room.north {
            Wall::Solid => print!(" ----"),
            Wall::Magical{ .. } => print!(" ++++"),
            Wall::Opening => print!("     ")
        }
    }
    println!();
    for row in 0..board.len() {        
        for col in 0..board[0].len() {
            if col == 0 {
                match board[row][col].west {
                    Wall::Solid => print!("|"),
                    Wall::Magical { .. } => print!("+"),
                    Wall::Opening => print!(" ")
                }
            }
            let pos = Position::new(col as i32, row as i32, board);
            let anyone = last_occupant(&pos, players);
            let character = match anyone {
                Some(occupant) => match *occupant {
                    Player::Explorer(_) => "E",
                    Player::Gnome(_) => "G",
                    Player::Leprechaun(_) => "L"
                },
                None => " "
            };
            match board[row][col].east {
                Wall::Solid => print!("  {} |", character),
                Wall::Magical { .. } => print!("  {} +", character),
                Wall::Opening => print!("  {}  ", character)
            }
        }
        println!();
        for room in board[row].iter() {
            match room.south {
                Wall::Solid => print!(" ----"),
                Wall::Magical { .. } => print!(" ++++"),
                Wall::Opening => print!("     ")
            }
        }
        println!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    x: i32,
    y: i32
}

impl Position {
    pub fn new(x: i32, y: i32, board: &Board) -> Position {
        if !xy_in_bounds(&x, &y, board) {
            panic!("position out of bounds")
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
            _player = Player::Explorer(exp_scavenge(exp, board))
        },
        Player::Gnome(gnome) => {
            _player = Player::Gnome(gnome_scavenge(gnome, board))
        },
        Player::Leprechaun(lep) => {
            _player = Player::Leprechaun(lep)
        }
    }

    _player
}

// TODO
pub fn is_opening(pos: &Position, direction: &players::Direction, board: &Board) -> bool {
    false
}

// TODO
pub fn open_sesame(word: &String, source: &Position, target: &Position, board: &Board) -> bool {
    false
}

fn pos_to_room(pos: &Position, board: &Board) -> Room {
    board[pos.y as usize][pos.x as usize].clone()
}

fn display_room_contents(room: &Room) {
    println!("room contains:");

    if room.contents.is_empty() {
        println!("nothing\n");
        return
    }

    for thing in room.contents.iter() {
        println!("{:?}", thing)
    }

    println!()
}

fn xy_in_bounds(x: &i32, y: &i32, board: &Board) -> bool {
    x_in_bounds(x, board) && y_in_bounds(y, board)
}

fn x_in_bounds(x: &i32, board: &Board) -> bool {
    *x >= 0 && *x < board[0].len() as i32
}

fn y_in_bounds(y: &i32, board: &Board) -> bool {
    *y >= 0 && *y < board.len() as i32
}

fn exp_scavenge(data: ExplorerData, board: &mut Board) -> ExplorerData {
    let mut exp = data;
    let pos = players::get_exp_pos(&exp);
    let mut input = String::new();
    
    if room_has_torch(&pos, board) || inventory::exp_has_torch(&exp) {
        let empty = pos_to_room(&pos, board).contents.is_empty();

        while !empty {
            inventory::display_exp_things(&exp);
            display_room_contents(&pos_to_room(&pos, board));

            println!("Enter letter command: Pick up [F]ood [C]oins tele[P]orter [T]orch or [D]one");
            
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
                        'F' => { exp_pick_up_food(&mut exp, board); players::exp_eat_food(&mut exp) },
                        'C' => exp_pick_up_coins(&mut exp, board),
                        'P' => exp_pick_up_teleporter(&mut exp, board),
                        'T' => exp_pick_up_torch(&mut exp, board),
                        'D' => break,
                        _ => println!("Invalid command")
                    }
                },
                None => println!("Ignoring leading whitespace")
            }

            input.clear()
        }
    } else {
        inventory::display_exp_things(&exp)
    }

    exp
}

fn last_occupant<'a, 'b>(pos: &'a Position, players: &'b Players) -> Option<&'b Player> {
    players.iter()
           .filter(|&player| players::is_occupant(player, pos))
           .last()
}

fn pick_up_thing(board: &Board, pos: &Position, index: usize) -> (Thing, Room) {
    let mut room = board[pos.y as usize][pos.x as usize].clone();
    let thing = room.pick_up_thing(index);

    (thing, room)
}

// TODO
fn gnome_scavenge(data: GnomeData, board: &mut Board) -> GnomeData {
    let mut gnome = data;

    // Your code goes here.

    gnome
}

// TODO
fn room_has_torch(pos: &Position, board: &Board) -> bool {
    false
}

// TODO
fn exp_pick_up_food(exp: &mut ExplorerData, board: &mut Board) {
    println!("Feature not implemented.")
}

// TODO
fn exp_pick_up_coins(exp: &mut ExplorerData, board: &mut Board) {
    println!("Feature not implemented.")
}

// TODO
fn exp_pick_up_teleporter(exp: &mut ExplorerData, board: &mut Board) {
    println!("Feature not implemented.")
}

// TODO
fn exp_pick_up_torch(exp: &mut ExplorerData, board: &mut Board) {
    println!("Feature not implemented.")
}
