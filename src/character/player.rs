use firecore_util::Direction;
use firecore_util::GlobalPosition;

use super::Character;
use super::CharacterProperties;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct PlayerCharacter {

	pub position: GlobalPosition,
	pub properties: CharacterProperties,

	pub input_frozen: bool,

}

impl PlayerCharacter {

    pub fn do_move(&mut self, delta: f32) -> bool {
        if !self.position.local.offset.is_none() {
            if let Some(offset) = self.position.local.offset.update(delta * self.properties.speed, &self.position.local.direction) {
                self.position.local.coords += offset;
                self.update_sprite();
                true
            } else {
                false
            }
        } else {
            false
        }        
    }

    pub fn freeze_input(&mut self) {
        self.input_frozen = true;
        self.stop_move();
    }

}

impl Character for PlayerCharacter {

    fn update_sprite(&mut self) {
        if self.properties.sprite_index == 0 {
			self.properties.sprite_index = 2;
		} else {
			self.properties.sprite_index = 0;
		}
    }

    fn on_try_move(&mut self, direction: Direction) {
        self.position.local.direction = direction;
		// self.update_sprite();
    }

    fn stop_move(&mut self) {
        self.properties.moving = false;
		self.properties.running = false;
		self.position.local.offset.reset();
		self.properties.reset_speed();
    }

    fn freeze(&mut self) {
		self.properties.frozen = true;
        self.stop_move();
    }

    fn unfreeze(&mut self) {
        self.input_frozen = false;
        self.properties.frozen = false;
    }

    fn is_frozen(&self) -> bool {
        self.properties.frozen || self.input_frozen
    }

    // fn start_move_to(&mut self, destination: Destination) -> bool {
    //     false
    // }

    fn should_move_to_destination(&self) -> bool {
        if let Some(offset) = self.properties.destination.as_ref() {
            self.position.local.coords != offset.coords
        } else {
            false
        }
    }

    fn move_to_destination(&mut self, delta: f32) {
        if let Some(destination) = self.properties.destination {

            if self.position.local.coords.y == destination.coords.y {
                self.position.local.direction = if self.position.local.coords.x < destination.coords.x {
                    Direction::Right
                } else {
                    Direction::Left
                };
            } else if self.position.local.coords.x == destination.coords.x {
                self.position.local.direction = if self.position.local.coords.y < destination.coords.y {
                    Direction::Down
                } else {
                    Direction::Up
                };
            }

            if let Some(offset) = self.position.local.offset.update(delta * self.properties.base_speed, &self.position.local.direction) {
                self.position.local.coords += offset;
                self.update_sprite();
                if self.position.local.coords == destination.coords {
                    if let Some(direction) = destination.direction {
                        self.position.local.direction = direction;
                    }
                }
            }            
        }
    }
}