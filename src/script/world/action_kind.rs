use serde::{Deserialize, Serialize};

use firecore_pokedex::pokemon::instance::PokemonInstance;
use firecore_util::Direction;
use firecore_util::music::Music;
use firecore_util::sound::Sound;
use firecore_util::text::MessageSet;
use firecore_util::Coordinate;

use crate::character::movement::Destination;
use crate::character::npc::NPC;
use crate::warp::WarpDestination;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldActionKind {

    Wait(f32),

    PlayMusic(Music),
    PlayMapMusic,
    PlaySound(Sound),

    PlayerFreeze,
    PlayerUnfreeze,
    PlayerLook(Direction),
    PlayerMove(Coordinate),
    PlayerGivePokemon(PokemonInstance),

    Warp(WarpDestination),

    NPCSpawn {
        id: u8,
        npc: NPC,
    },
    NPCLook(u8, Direction),
    NPCMove(u8, Destination),
    NPCLeadPlayer(u8, Destination),
    NPCMoveToPlayer(u8),
    NPCInteract(u8),
    NPCBattle(u8),
    NPCDespawn(u8),

    DisplayText(MessageSet),

    // Battle(BattleType, ),

}