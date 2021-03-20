use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};
use firecore_util::{Entity, Timer, BoundingBox};

use std::collections::VecDeque;

mod condition;
mod action_kind;

pub use self::condition::Condition;
pub use self::action_kind::WorldActionKind;

use super::ScriptId;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldScript {
    
    pub identifier: ScriptId,

    pub location: Option<BoundingBox>,

    pub conditions: Vec<Condition>,

    actions: VecDeque<WorldActionKind>,

    
    #[serde(skip)]
    alive: bool,

    #[serde(skip)]
    pub actions_clone: VecDeque<WorldActionKind>,

    #[serde(skip)]
    pub timer: Timer,

}

impl WorldScript {

    pub fn test_pos(&self, coords: &Coordinate) -> bool {
        self.location.as_ref().map(|location| location.in_bounds(coords)).unwrap_or(true)
    }

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