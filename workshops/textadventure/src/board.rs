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

enum Wall {
    Solid,
    Opening,
    Magical { word: String }
}

struct Room {
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
    contents: Vec<Thing>
}

// FIXM Add contents to rooms
pub fn build_board() -> Board {
    use self::Wall::*;

    [[Room {north: Solid, east: Solid, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Opening, west: Opening,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Solid, west: Opening,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Solid, west: Opening,
            contents: vec![]},
      Room {north: Solid, east: Solid, south: Opening, west: Solid,
            contents: vec![]}
     ],
     [Room {north: Opening, east: Opening, south: Solid, west: Solid,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Solid, west: Opening,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Opening, west: Opening,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Opening, west: Opening,
            contents: vec![]}
     ],
     [Room {north: Solid, east: Opening, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Solid, south: Opening, west: Opening,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Solid, west: Solid,
            contents: vec![]}
     ],
     [Room {north: Opening, east: Solid, south: Opening, west: Solid,
            contents: vec![]},
      Room {north: Opening, east: Opening, south: Solid, west: Solid,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Solid, west: Opening,
            contents: vec![]},
      Room {north: Opening, east: Opening, south: Solid, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Solid, south: Opening, west: Opening,
            contents: vec![]}
     ],
     [Room {north: Opening, east: Opening, south: Solid, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Solid, south: Solid, west: Opening,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Solid, west: Solid,
            contents: vec![]},
      Room {north: Solid, east: Opening, south: Solid, west: Opening,
            contents: vec![]},
      Room {north: Opening, east: Solid, south: Solid, west: Opening,
            contents: vec![]}
     ]]
}

pub fn display_map(board: &Board, players: &Players) {
    for room in board[0].iter() {
        match room.north {
            Wall::Solid => print!(" ----"),
            Wall::Magical{ref word} => print!(" ~~~~"),
            Wall::Opening => print!("     ")
        }
    }
    println!();
    for row in board.iter() {        
        for col in 0..board.len() {
            if col == 0 {
                match row[col].west {
                    Wall::Solid => print!("|"),
                    Wall::Magical { ref word } => print!(":"),
                    Wall::Opening => print!(" ")
                }
            }
            match row[col].east {
                Wall::Solid => print!("    |"),
                Wall::Magical { ref word } => print!("    :"),
                Wall::Opening => print!("     ")
            }
        }
        println!();
        for room in row.iter() {
            match room.south {
                Wall::Solid => print!(" ----"),
                Wall::Magical { ref word } => print!(" ~~~~"),
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

fn pos_to_room<'a, 'b>(pos: &'a Position, board: &'b Board) -> &'b Room {
    &board[pos.y as usize][pos.x as usize]
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
    *x < board.len() as i32
}

fn y_in_bounds(y: &i32, board: &Board) -> bool {
    *y < board[0].len() as i32
}

fn exp_scavenge(data: ExplorerData, board: &mut Board) -> ExplorerData {
    let mut exp = data;
    let pos = players::get_exp_pos(&exp);
    let mut input = String::new();
    
    if room_has_torch(&pos, board) || inventory::exp_has_torch(&exp) {
        let empty = pos_to_room(&pos, board).contents.is_empty();

        while !empty {
            inventory::display_exp_things(&exp);
            display_room_contents(pos_to_room(&pos, board));

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
                        'F' => { exp_pick_up_food(&mut exp, board); exp_eat_food(&mut exp) },
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
fn room_has_torch(pos: &Position, board: &Board) -> bool {
    false
}

// TODO
fn exp_pick_up_food(exp: &mut ExplorerData, board: &mut Board) {
    println!("Feature not implemented.")
}

// TODO
fn exp_eat_food(exp: &mut ExplorerData) {
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
