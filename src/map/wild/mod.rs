use firecore_pokedex::pokemon::battle::BattlePokemon;
use serde::{Deserialize, Serialize};

use crate::TileId;

use self::table::WildPokemonTable;

pub mod encounter;
pub mod table;

#[derive(Serialize, Deserialize)]
pub struct WildEntry {

    pub tiles: Option<Vec<TileId>>,
    pub table: WildPokemonTable,

}

pub trait GenerateWild {

    fn generate(&self) -> BattlePokemon;

}