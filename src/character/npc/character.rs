use firecore_util::Coordinate;
use firecore_util::Direction;

use crate::character::Character;
use crate::character::movement::Destination;

use super::NPC;

impl NPC {

    pub fn walk_to(&mut self, to: &Coordinate) {
        self.properties.character.destination = Some(Destination::to(&self.position, to));
    }

    pub fn walk_next_to(&mut self, to: &Coordinate) {
        self.properties.character.destination = Some(Destination::next_to(&self.position, to));
    }

}

fn change_sprite_index(sprite_index: &mut u8) {
    if 0.eq(sprite_index) {
        *sprite_index = 2;
    } else {
        *sprite_index = 0;
    }
}

impl Character for NPC {

    fn on_try_move(&mut self, direction: Direction) {
        self.position.direction = direction;
        change_sprite_index(&mut self.properties.character.sprite_index);
    }

    fn freeze(&mut self) {
        self.properties.character.frozen = true;
        self.properties.character.moving = false;
		self.properties.character.running = false;
		self.properties.character.reset_speed();
    }

    fn should_move_to_destination(&self) -> bool {
        if let Some(destination) = self.properties.character.destination.as_ref() {
            self.position.coords != destination.coords
        } else {
            false
        }
    }

    fn move_to_destination(&mut self, delta: f32) {

        if let Some(destination) = self.properties.character.destination.as_ref() {

            if self.position.coords.y == destination.coords.y {
                self.position.direction = if self.position.coords.x < destination.coords.x {
                    Direction::Right
                } else {
                    Direction::Left
                };
            } else if self.position.coords.x == destination.coords.x {
                self.position.direction = if self.position.coords.y < destination.coords.y {
                    Direction::Down
                } else {
                    Direction::Up
                };
            }

            let offsets = self.position.direction.offset_f32();
            let offset = 60.0 * self.properties.character.speed * delta;
            self.position.offset.x += offsets.x * offset;
            self.position.offset.y += offsets.y * offset;

            if self.position.offset.y * offsets.y >= 16.0 {
                self.position.coords.y += offsets.y as isize;
                self.position.offset.y = 0.0;
                change_sprite_index(&mut self.properties.character.sprite_index);
                if self.position.coords == destination.coords {
                    if let Some(direction) = destination.direction {
                        self.position.direction = direction;
                    }
                }
            }
            
            if self.position.offset.x * offsets.x >= 16.0 {
                self.position.coords.x += offsets.x as isize;
                self.position.offset.x = 0.0;
                change_sprite_index(&mut self.properties.character.sprite_index);
                if self.position.coords == destination.coords {
                    if let Some(direction) = destination.direction {
                        self.position.direction = direction;
                    }
                }
            }
            
        }
    
    }

}