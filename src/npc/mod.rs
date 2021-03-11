// use crate::io::data::player::PlayerData;
// use crate::io::data::text::MessageSet;
use firecore_util::Position;
use firecore_util::text::MessageSet;
use serde::{Deserialize, Serialize};
use self::movement::NPCDestination;
use self::trainer::Trainer;

// use super::NpcTextures;
// use super::player::Player;

pub mod movement;
pub mod trainer;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPC {

    pub identifier: NPCIdentifier,


    pub position: Position, // Home position

    #[serde(default)]
    pub movement_type: MovementType,

    #[serde(default)]
    pub message_set: MessageSet,

    #[serde(default = "default_speed")]
    pub speed: f32,


    pub trainer: Option<Trainer>,


    #[serde(skip)]
    pub offset: Option<NPCDestination>, // Offset from home position, see if changing the struct to something that uses variables better would help

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPCIdentifier {

    pub name: String,
    pub npc_type: String,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MovementType {

    Still,
    LookAround,
    WalkUpAndDown(isize),

}

// impl NPC {

//     pub fn interact(&mut self, direction: Option<Direction>, player: &mut Player) {
//         if let Some(direction) = direction {
//             self.position.direction = direction.inverse();
//         }
//         if self.trainer.is_some() {
//             macroquad::prelude::info!("Trainer battle with {}", &self.identifier.name);
//             self.walk_next_to(&player.position.local.coords);
//             player.freeze();
//         }
//     }

//     pub fn after_interact(&mut self, map_name: &String) {
//         if let Some(mut player_data) = macroquad::prelude::collections::storage::get_mut::<PlayerData>() {
//             // macroquad::prelude::info!("Finished interacting with {}", self.identifier.name);
//             player_data.world_status.get_or_create_map_data(map_name).battle(&self);
//         } else {
//             macroquad::prelude::warn!("Could not get player data!");
//         }
//     }

//     pub fn render(&self, npc_textures: &NpcTextures, screen: &super::RenderCoords) {
//         let x = ((self.position.coords.x + screen.x_tile_offset) << 4) as f32 - screen.focus.x + self.position.offset.x;
//         let y = ((self.position.coords.y - 1 + screen.y_tile_offset) << 4) as f32 - screen.focus.y + self.position.offset.y;
//         if let Some(twt) = npc_textures.get(&self.identifier.npc_type) {
//             let tuple = twt.of_direction(self.position.direction);
//             crate::util::graphics::draw_flip(tuple.0, x, y, tuple.1);
//         } else {
//             crate::util::graphics::draw_rect([1.0, 0.0, 0.0, 1.0], x, y + crate::util::TILE_SIZE as f32, 16, 16);
//         }
//     }

// }

impl Default for MovementType {
    fn default() -> Self {
        Self::Still
    }
}

// impl Default for NPC {
//     fn default() -> Self {
//         Self {
//             identifier: NPCIdentifier::default(),
//             position: Position::default(),
//             movement_type: MovementType::default(),
//             message: MessageSet::default(),
//             speed: default_speed(),
//             trainer: None,
//             offset: None,
//         }
//     }
// }

const fn default_speed() -> f32 {
    1.0
}

impl Default for NPCIdentifier {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            npc_type: String::from("youngster"),
        }
    }
}