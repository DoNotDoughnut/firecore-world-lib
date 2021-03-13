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

    pub fn safe_tile(&self, x: isize, y: isize) -> Option<u16> {
        if self.map.in_bounds(x, y) {
            Some(self.map.tile(x, y))
        } else {
            None
        }
    }

}

impl World for WorldChunk {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        self.map.in_bounds(x, y)
    }

    fn tile(&self, x: isize, y: isize) -> TileId {
        self.map.tile(x, y)
    }

    fn walkable(&self, x: isize, y: isize) -> MovementId {
        if self.in_bounds(x, y) {
            return self.map.walkable(x, y);
        } else {
            1
        }        
    }

    fn check_warp(&self, coords: &Coordinate) -> Option<WarpEntry> {
        self.map.check_warp(coords)
    }

}