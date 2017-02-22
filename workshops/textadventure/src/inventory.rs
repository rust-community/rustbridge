use players::Players;
use players::Player;
use players::ExplorerData;
use players::GnomeData;
use players::LeprechaunData;
use players::is_occupant;
use players::get_gnome_position;
use board;
use std::io;

pub enum Thing { Food, Torch, Teleporter }

pub fn encounter_player(player: Player, others: &mut Players) -> Player {
    let _player : Player;

    match player {
        Player::Explorer(data) => {
            _player = Player::Explorer(encounter_explorer(data, others));
        },
        Player::Gnome(data) => {
            _player = Player::Gnome(encounter_gnome(data, others));
        },
        Player::Leprechaun(data) => {
            _player = Player::Leprechaun(encounter_leprechaun(data, others));
        }
    }

    _player
}

pub fn encounter_gnome(data: GnomeData, others: &mut Players) -> GnomeData {
    let mut _data = data;

    let shake_down = |occupant: Player| {
        let _occupant : Player;
        
        match occupant {
            Player::Explorer(data) => {
                _occupant = Player::Explorer(data);  // TODO
            },
            Player::Gnome(data) => _occupant = Player::Gnome(data),
            Player::Leprechaun(data) => _occupant = Player::Leprechaun(data)
        }

        _occupant
    };

    exchange_with_other_occupants(get_gnome_position(&_data), others, shake_down);
    
    _data
}

pub fn encounter_explorer(data: ExplorerData, others: &mut Players) -> ExplorerData {
    unimplemented!();
}

pub fn encounter_leprechaun(data: LeprechaunData, others: &mut Players) -> LeprechaunData {
    unimplemented!();
}

fn exchange_with_other_occupants<F>(pos: &board::Position, others: &mut Players, exchange: F)
where F: Fn(Player) -> Player {
    let rotation = others.len() as i32;
    let mut index = 0;

    while index < rotation {
        match others.pop_front() {
            Some(other) => {
                index += 1;
                if is_occupant(&other, pos) {
                    let exchanged = exchange(other);
                    others.push_back(exchanged);
                } else {
                    others.push_back(other);
                }
            },
            None => panic!("missing other")
        }
    }
}
