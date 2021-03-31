use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};

use crate::MovementId;
use crate::TileId;

use super::World;
use super::WorldMap;
use super::warp::WarpDestination;

pub mod manager;

#[derive(Default, Deserialize, Serialize)]
pub struct WorldMapSet {

    pub name: String,
    pub maps: Vec<WorldMap>,
    
    #[serde(skip)]
    pub current_map: usize,

}

impl WorldMapSet {

    pub fn new(name: String, maps: Vec<WorldMap>) -> Self {
        Self {
            name,
            maps,
            current_map: 0,
        }
    }

    pub fn map(&self) -> &WorldMap {
        self.maps.get(self.current_map).unwrap()
    }

    pub fn map_mut(&mut self) -> &mut WorldMap {
        self.maps.get_mut(self.current_map).unwrap()
    }

}

impl World for WorldMapSet {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.maps[self.current_map].in_bounds(coords)
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.maps[self.current_map].tile(coords)
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        if self.in_bounds(coords) {
            self.maps[self.current_map].walkable(coords)
        } else {
            1
        }
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        self.maps[self.current_map].check_warp(coords)
    }

}