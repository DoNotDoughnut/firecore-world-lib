use serde::{Deserialize, Serialize};
use firecore_pokedex::PokemonId;

#[derive(Debug, Deserialize, Serialize)]
pub enum Condition {

    WorldEvent{
        id: String, 
        happened: bool, 
        activate: bool,
    },

    PlayerPokemonAny(Vec<PokemonId>),
    PlayerPokemonAll(Vec<PokemonId>),

}