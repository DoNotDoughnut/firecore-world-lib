use serde::{Deserialize, Serialize};
use ahash::AHashSet as HashSet;
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_util::battle::BattleScreenTransitions;

use super::NPCId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trainer {

    pub tracking_length: Option<u8>,
    pub encounter_message: Vec<Vec<String>>,

    #[serde(default)]
    pub battle_transition: BattleScreenTransitions,

    pub party: PokemonParty,

    #[serde(default)]
    pub victory_message: Vec<String>,
    #[serde(default)]
    pub disable_others: HashSet<NPCId>,
    pub worth: u16,

}