use firecore_util::Coordinate;
use firecore_util::Direction;

use crate::character::movement::Destination;

use super::NPC;

impl NPC {

    pub fn walk_to(&mut self, to: &Coordinate) {
        self.properties.destination = Some(Destination::to(&self.position, to));
    }

    pub fn walk_next_to(&mut self, to: &Coordinate) {
        self.properties.destination = Some(Destination::next_to(&self.position, to));
    }

    pub fn should_move(&self) -> bool {
        if let Some(destination) = self.properties.destination.as_ref() {
            self.position.coords != destination.coords
        } else {
            false
        }
    }

    pub fn do_move(&mut self, delta: f32) {

        if let Some(destination) = self.properties.destination.as_mut() {

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
            let offset = 60.0 * self.properties.speed * delta;
            self.position.offset.x += offsets.x * offset;
            self.position.offset.y += offsets.y * offset;

            if self.position.offset.y * offsets.y >= 16.0 {
                self.position.coords.y += offsets.y as isize;
                self.position.offset.y = 0.0;
                if self.position.coords == destination.coords {
                    self.position.direction = destination.direction;
                }
            }
            
            if self.position.offset.x * offsets.x >= 16.0 {
                self.position.coords.x += offsets.x as isize;
                self.position.offset.x = 0.0;
                if self.position.coords == destination.coords {
                    self.position.direction = destination.direction;
                }
            }
            
        }
    
    }

}