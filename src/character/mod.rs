use firecore_util::{Direction, Destination};
use serde::{Deserialize, Serialize};

pub mod movement;
pub mod npc;
pub mod sprite;
pub mod player;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterProperties {

    #[serde(default = "default_speed")]
    pub base_speed: f32,

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

    // #[deprecated]
    #[serde(skip)]
    pub destination: Option<Destination>,

    // #[serde(skip)]
    // pub destination: Option<DestinationPath>,

}

pub trait Character {

    fn update_sprite(&mut self);

    fn on_try_move(&mut self, direction: Direction);

    fn stop_move(&mut self);

    fn freeze(&mut self);

    fn unfreeze(&mut self);

    fn is_frozen(&self) -> bool;

    // fn start_move_to(&mut self, destination: Destination) -> bool;

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