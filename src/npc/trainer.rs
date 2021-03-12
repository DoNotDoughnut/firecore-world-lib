use serde::{Deserialize, Serialize};
use firecore_pokedex::pokemon::party::PokemonParty;

use crate::{BattleType, BattleScreenTransitions};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trainer {

    pub trainer_type: TrainerType,

    pub tracking_length: Option<usize>,
    pub encounter_music: Option<firecore_audio::music::Music>,
    pub encounter_message: Vec<Vec<String>>, // MessageSet

    pub battle_transition: Option<BattleScreenTransitions>,

    pub party: PokemonParty,

    #[serde(default)]
    pub victory_message: Vec<String>,
    #[serde(default)]
    pub disable_others: Vec<String>,
    pub worth: u16,

}

// pub fn npc_type_string(npc_type: &str) -> String {
//     let mut name = String::new();
//     let mut space_before = true;
//     for mut char in npc_type.chars() {
//         if space_before {
//             space_before = false;
//             char.make_ascii_uppercase();
//         }
//         if char == ' ' {
//             space_before = true;
//         }
//         name.push(char);
//     }
//     macroquad::prelude::info!("{}", name);
//     return name;
// }

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum TrainerType { // To - do: I don't like having this in an enum, I'd preferably parse strings instead

    None,
    Camper,
    Youngster,
    Lass,
    BugCatcher,
    GymLeader,
    
}

impl TrainerType {

    pub fn battle_type(&self) -> BattleType {
        match self {
            TrainerType::GymLeader => BattleType::GymLeader,
            _ => BattleType::Trainer,
        }
    }

}

impl std::fmt::Display for TrainerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TrainerType::None => "Trainer",
            TrainerType::Camper => "Camper",
            TrainerType::Youngster => "Youngster",
            TrainerType::Lass => "Lass",
            TrainerType::BugCatcher => "Bug Catcher",
            TrainerType::GymLeader => "Gym Leader",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerData {

    pub name: String,
    pub npc_type: String,
    #[serde(default)]
    pub transition: BattleScreenTransitions,

}