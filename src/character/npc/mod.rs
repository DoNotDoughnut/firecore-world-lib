use firecore_util::Position;
use firecore_util::text::Message;
use serde::{Deserialize, Serialize};

use super::CharacterProperties;
use super::movement::MovementType;
use self::trainer::Trainer;

pub mod npc_type;

pub mod trainer;

pub mod character;
pub mod interact;

pub type NPCId = u8;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPC {

    pub identifier: NPCIdentifier,

    pub position: Position,

    #[serde(default)]
    pub properties: NPCProperties, // To - do: Make non-optional (for 0.4.X)

    pub trainer: Option<Trainer>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPCIdentifier {

    pub index: NPCId,
    pub name: String,
    pub npc_type: String,

}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct NPCProperties {

    #[serde(default)]
    pub character: CharacterProperties,

    #[serde(default)]
    pub movement: MovementType,

    pub message: Option<Vec<Message>>,

}