use players::Player;
use players::ExplorerData;
use players::GnomeData;
use players::LeprechaunData;
use players::Players;

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

pub fn encounter_explorer(data: ExplorerData, others: &mut Players) -> ExplorerData {
    unimplemented!();
    ExplorerData{ x: data.x, y: data.y, energy: data.energy }
}

pub fn encounter_gnome(data: GnomeData, others: &mut Players) -> GnomeData {
    unimplemented!();
    GnomeData{ x: data.x, y: data.y, energy: data.energy }
}

pub fn encounter_leprechaun(data: LeprechaunData, others: &mut Players) -> LeprechaunData {
    unimplemented!();
    LeprechaunData{ x: data.x, y: data.y, energy: data.energy }
}
