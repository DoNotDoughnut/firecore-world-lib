use serde::{Deserialize, Serialize};

use self::wild_pokemon_table::WildPokemonTable;

pub mod wild_pokemon_encounter;
pub mod wild_pokemon_table;

#[derive(Serialize, Deserialize)]
pub struct WildEntry {

    pub tiles: Option<Vec<u16>>,
    pub table: WildPokemonTable,

}