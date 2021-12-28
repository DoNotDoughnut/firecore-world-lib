use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use pokedex::pokemon::owned::SavedPokemon;

use crate::{map::manager::state::WorldMapState, positions::Location};

use super::{
    npc::{Npc, NpcId},
    trainer::Trainer,
    Character,
};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub location: Location,
    pub character: Character,
    pub trainer: Trainer,

    pub pc: Vec<SavedPokemon>,
    pub world: WorldMapState,

    pub rival: String,
    pub input_frozen: bool,
    pub ignore: bool,
}

impl Deref for PlayerCharacter {
    type Target = Character;

    fn deref(&self) -> &Self::Target {
        &self.character
    }
}

impl DerefMut for PlayerCharacter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.character
    }
}

impl PlayerCharacter {
    pub fn find_battle(&mut self, map: &Location, id: &NpcId, npc: &mut Npc) -> bool {
        if self.world.npc.active.is_none()
            && !self.world.battle.battled(map, id)
            && npc.find_character(&mut self.character)
        {
            self.world.npc.active = Some(*id);
            true
        } else {
            false
        }
    }

    /// does not cover cases where pokemon cannot be sent to pc
    pub fn give_pokemon(&mut self, pokemon: SavedPokemon) {
        match self.trainer.party.is_full() {
            true => self.pc.push(pokemon),
            false => self.trainer.party.push(pokemon),
        }
    }
}
