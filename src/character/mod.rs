use firecore_util::Direction;
use serde::{Deserialize, Serialize};

use self::movement::Destination;

pub mod movement;
pub mod npc;
pub mod sprite;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterProperties {

    #[serde(default = "default_speed")]
    base_speed: f32,

    #[serde(skip, default = "default_speed")]
    pub speed: f32,

    #[serde(skip)]
    pub sprite_index: u8,

    #[serde(skip)]
    pub moving: bool,

    #[serde(skip)]
    pub running: bool,

    #[serde(skip)]
    pub frozen: bool,

    #[serde(skip)]
    pub noclip: bool,

    #[serde(skip)]
    pub destination: Option<Destination>,


}

pub trait Character {

    fn on_try_move(&mut self, direction: Direction);

    fn freeze(&mut self);

    fn should_move_to_destination(&self) -> bool;

    fn move_to_destination(&mut self, delta: f32);

}

impl CharacterProperties {

    pub fn reset_speed(&mut self) {
        self.speed = self.base_speed;
    }

}

impl Default for CharacterProperties {
    fn default() -> Self {
        Self {
            base_speed: default_speed(),
            speed: default_speed(),
            sprite_index: 0,
            moving: false,
            running: false,
            frozen: false,
            noclip: false,
            destination: None,
        }
    }
}

const fn default_speed() -> f32 {
    1.0
}