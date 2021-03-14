use serde::{Deserialize, Serialize};

use self::movement::Destination;

pub mod movement;
pub mod npc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterProperties {

    #[serde(default = "default_speed")]
    pub speed: f32,

    #[serde(skip)]
    pub sprite_index: u8,

    #[serde(skip)]
    pub moving: bool,

    #[serde(skip)]
    pub running: bool,

    // #[serde(skip)]
    // pub noclip: bool,

    #[serde(skip)]
    pub destination: Option<Destination>,


}

impl Default for CharacterProperties {
    fn default() -> Self {
        Self {
            speed: 1.0,
            sprite_index: 0,
            moving: false,
            running: false,
            destination: None,
        }
    }
}

const fn default_speed() -> f32 {
    1.0
}