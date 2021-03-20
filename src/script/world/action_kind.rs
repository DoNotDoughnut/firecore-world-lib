use serde::{Deserialize, Serialize};

use firecore_util::{Direction, Destination};
use firecore_util::text::MessageSet;

use firecore_audio_lib::music::MusicName;
use firecore_audio_lib::sound::Sound;

use firecore_pokedex::pokemon::instance::PokemonInstance;

use crate::character::npc::{NPC, NPCId};
use crate::map::warp::WarpDestination;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldActionKind {

    PlayMusic(MusicName),
    PlayMapMusic,
    PlaySound(Sound),

    PlayerFreezeInput,
    PlayerUnfreezeInput,
    PlayerUnfreeze,
    PlayerLook(Direction),
    PlayerMove(Destination),
    PlayerGivePokemon(PokemonInstance),

    NPCSpawn(NPC),
    NPCLook(NPCId, Direction),
    NPCMove(NPCId, Destination),
    NPCLeadPlayer(NPCId, Destination),
    NPCMoveToPlayer(NPCId),
    NPCInteract(NPCId),
    NPCBattle(NPCId),
    NPCRespawn(NPCId),
    NPCDespawn(NPCId),

    Wait(f32),

    DisplayText(MessageSet),

    Warp(WarpDestination, bool), // bool: change music

}