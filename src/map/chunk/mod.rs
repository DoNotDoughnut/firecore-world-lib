use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::MovementId;
use crate::TileId;
use crate::World;
use crate::warp::WarpEntry;

use super::WorldMap;


pub mod world_chunk_map;

#[derive(Default, Deserialize, Serialize)]
pub struct WorldChunk {

    pub index: u16,

    pub x: isize,
    pub y: isize,

    pub map: WorldMap,

    pub connections: SmallVec<[u16; 6]>,

}

impl WorldChunk {

    pub fn safe_tile(&self, coords: &Coordinate) -> Option<u16> {
        if self.map.in_bounds(coords) {
            Some(self.map.tile(coords))
        } else {
            None
        }
    }

}

impl World for WorldChunk {

    fn in_bounds(&self, coords: &Coordinate) -> bool {
        self.map.in_bounds(coords)
    }

    fn tile(&self, coords: &Coordinate) -> TileId {
        self.map.tile(coords)
    }

    fn walkable(&self, coords: &Coordinate) -> MovementId {
        if self.in_bounds(coords) {
            return self.map.walkable(coords);
        } else {
            1
        }        
    }

    fn check_warp(&self, coords: &Coordinate) -> Option<WarpEntry> {
        self.map.check_warp(coords)
    }

}