use firecore_audio_lib::music::MusicName;
use firecore_util::Position;
use firecore_util::text::MessageSet;
use serde::{Deserialize, Serialize};
use crate::battle::BattleType;

use super::CharacterProperties;
use super::movement::MovementType;
use self::trainer::Trainer;

pub mod trainer;

pub mod character;
pub mod interact;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPC {

    pub identifier: NPCIdentifier,

    pub position: Position,

    #[serde(default)]
    pub properties: NPCProperties,

    pub trainer: Option<Trainer>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPCIdentifier {

    pub name: String,
    pub npc_type: String,

}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct NPCProperties {

    #[serde(default)]
    pub character: CharacterProperties,

    #[serde(default)]
    pub movement: MovementType,

    pub message: Option<MessageSet>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPCType {

    pub identifier: String,
    pub sprite_indexes: u8,
    pub trainer: Option<TrainerType>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrainerType {

    pub battle_type: BattleType,
    pub encounter_music: MusicName,

}