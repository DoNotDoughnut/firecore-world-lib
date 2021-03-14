use serde::{Deserialize, Serialize};

use firecore_util::{Position, Coordinate, Direction};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MovementType {

    Still,
    LookAround,
    WalkUpAndDown(isize),

}

impl Default for MovementType {
    fn default() -> Self {
        Self::Still
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Destination {

    pub coords: Coordinate,
    pub direction: Direction,

}

impl Destination {

    pub fn to(from: &Position, to: &Coordinate) -> Self {
        Self {
            coords: *to,
            direction: from.coords.towards(&to),
        }
    }

    pub fn next_to(from: &Position, to: &Coordinate) -> Self {
        let direction = from.coords.towards(to);
        let (x, y) = direction.inverse().tile_offset();
        Destination {
            coords: to.add(x, y),
            direction,
        }
    }

}