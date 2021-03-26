use firecore_pokedex::pokemon::{
    PokemonId,
    instance::PokemonInstance,
    data::StatSet,
    generate::Generate,
    random::RandomSet,
};

use super::encounter::WildPokemonEncounter;

pub static DEFAULT_ENCOUNTER_CHANCE: u8 = 21;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WildPokemonTable {
    pub encounter_ratio: u8,
    pub encounter: Option<[WildPokemonEncounter; 12]>,
}

impl WildPokemonTable {

    pub fn encounter_rate(&self) -> u8 {
        self.encounter_ratio
    }

    pub fn generate(&self) -> PokemonInstance { // maybe move into trait for firecore-pokedex
        match self.encounter {
            Some(encounter) => encounter[get_counter()].generate(),
            None => return PokemonInstance::generate(
                quad_rand::gen_range(0, firecore_pokedex::POKEDEX.len()) as PokemonId + 1, 
                1, 
                100, 
                Some(StatSet::random()), 
            ),
        }
    }

}

impl Default for WildPokemonTable {
    fn default() -> Self {
        Self {
            encounter_ratio: DEFAULT_ENCOUNTER_CHANCE,
            encounter: None,
        }
    }
}

pub static CHANCES: [usize; 12] = [20, 20, 10, 10, 10, 10, 5, 5, 4, 4, 1, 1];


fn get_counter() -> usize {
    let chance = quad_rand::gen_range(1, 100);
    let mut chance_counter = 0;
    let mut counter = 0;
    while chance > chance_counter {
        chance_counter += CHANCES[counter];
        counter+=1;            
    }
    return counter - 1;
}