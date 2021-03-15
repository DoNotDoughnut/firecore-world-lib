use serde::{Deserialize, Serialize};
use firecore_pokedex::pokemon::party::PokemonParty;

use crate::battle::BattleScreenTransitions;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trainer {

    pub tracking_length: Option<usize>,
    pub encounter_message: Vec<Vec<String>>, // MessageSet

    #[serde(default)]
    pub battle_transition: BattleScreenTransitions,

    pub party: PokemonParty,

    #[serde(default)]
    pub victory_message: Vec<String>,
    #[serde(default)]
    pub disable_others: Vec<String>,
    pub worth: u16,

}