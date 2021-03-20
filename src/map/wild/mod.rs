use serde::{Deserialize, Serialize};

use self::table::WildPokemonTable;

pub mod encounter;
pub mod table;

#[derive(Serialize, Deserialize)]
pub struct WildEntry {

    pub tiles: Option<Vec<u16>>,
    pub table: WildPokemonTable,

}