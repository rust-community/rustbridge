use std::io;

struct Player {}

impl Player {
    fn take_turn(&self) -> bool {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // TODO parse and process input

        return true;
    }
}

fn main() {
    println!("Welcome to Gnomes! Let's begin.");

    let player = Player {};  // user
    let gnome = Player {};  // NPC
    let leprechaun = Player {};  // NPC

    loop {
        if !player.take_turn() {  // not alive
            break;
        }

        gnome.take_turn();
        leprechaun.take_turn();
    }

    println!("You have died!  GAME OVER");
}
