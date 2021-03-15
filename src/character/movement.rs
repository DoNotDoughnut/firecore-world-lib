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

#[serde(from = "Coordinate")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Destination {

    pub coords: Coordinate,

    pub direction: Option<Direction>,

}

impl Destination {

    pub fn to(from: &Position, to: &Coordinate) -> Self {
        Self {
            coords: *to,
            direction: Some(from.coords.towards(&to)),
        }
    }

    pub fn next_to(from: &Position, to: &Coordinate) -> Self {
        let direction = from.coords.towards(to);
        let (x, y) = direction.inverse().tile_offset();
        Destination {
            coords: to.add(x, y),
            direction: Some(direction),
        }
    }

}

impl From<Coordinate> for Destination {
    fn from(coords: Coordinate) -> Self {
        Self {
            coords,
            direction: None,
        }
    }
}

impl From<Position> for Destination {
    fn from(pos: Position) -> Self {
       Self {
           coords: pos.coords,
           direction: Some(pos.direction),
       }
    }
}