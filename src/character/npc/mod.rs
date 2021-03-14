use firecore_util::Position;
use firecore_util::text::MessageSet;
use serde::{Deserialize, Serialize};
use super::CharacterProperties;
use super::movement::MovementType;
use self::trainer::Trainer;

pub mod trainer;
pub mod movement;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPC {

    pub identifier: NPCIdentifier,

    pub position: Position, // Home position

    #[serde(default)]
    pub properties: CharacterProperties,

    #[serde(default)]
    pub movement_type: MovementType,

    pub message: Option<MessageSet>,

    pub trainer: Option<Trainer>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPCIdentifier {

    pub name: String,
    pub npc_type: String,

}