use firecore_pokedex::pokemon::{
    PokemonId,
    Level,
    battle::BattlePokemon,
    data::StatSet,
    generate::GeneratePokemon,
    random::RandomSet,
};

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct WildPokemonEncounter {

    #[serde(rename = "pokemon_id")]
    pub pokemon: PokemonId,

    #[serde(rename = "min_level")]
    pub min: Level,

    #[serde(rename = "max_level")]
    pub max: Level,

}

impl super::GenerateWild for WildPokemonEncounter {
    fn generate(&self) -> BattlePokemon {
        BattlePokemon::generate(self.pokemon, self.min, self.max, Some(StatSet::random()))
    }
}