use std::collections::VecDeque;

use firecore_util::text::MessageSet;
use serde::{Deserialize, Serialize};

use firecore_util::{Coordinate, BoundingBox};
use firecore_util::Entity;
use crate::npc::NPC;

#[derive(Debug, Deserialize, Serialize)]
pub enum Condition {

    WorldEvent{
        id: String, 
        happened: bool, 
        activate: bool,
    },

}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldScript {

    #[serde(skip)]
    alive: bool,

    pub condition: WorldCondition,
    actions: VecDeque<WorldActionKind>,

    #[serde(skip)]
    pub actions_clone: VecDeque<WorldActionKind>,

    #[serde(skip)]
    pub timer: firecore_util::Timer,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldActionKind {

    Wait(f32),

    FreezePlayer,
    UnfreezePlayer,

    SpawnNPC {
        id: u8,
        npc: NPC,
    },
    MoveNPC {
        id: u8,
        pos: Coordinate,
    },
    MoveNPCToPlayer {
        id: u8,
    },
    DespawnNPC {
        id: u8,
    },
    DisplayText {
        message_set: MessageSet,
    },

}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldCondition {

    pub location: BoundingBox,
    pub event: Option<Condition>,
    // pub persistant: bool,

}

impl Entity for WorldScript {
    fn spawn(&mut self) {
        self.alive = true;
        self.actions_clone = self.actions.clone();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}