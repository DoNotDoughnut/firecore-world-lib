use firecore_util::Coordinate;
use serde::{Serialize, Deserialize};
use ahash::AHashMap as HashMap;

use crate::MovementId;
use crate::TileId;

use crate::map::World;
use crate::map::warp::WarpDestination;

use super::WorldMapSet;

#[derive(Default, Serialize, Deserialize)]
pub struct WorldMapSetManager {

    pub map_sets: HashMap<String, WorldMapSet>,
    
    #[serde(skip)]
    pub current_map_set: String,

}

impl WorldMapSetManager {

    pub fn set(&mut self, set: &String) {
        if self.map_sets.contains_key(set) {
            self.current_map_set = set.clone();
        }
    }

    pub fn set_index(&mut self, index: usize) {
        self.map_set_mut().current_map = index;
    }

    pub fn map_set(&self) -> &WorldMapSet {
        match self.map_sets.get(&self.current_map_set) {
            Some(map_set) => map_set,
            None => {
                panic!("Could not get current map set {}", self.current_map_set)
            }
        }
    }

    pub fn map_set_mut(&mut self) -> &mut WorldMapSet {
        self.map_sets.get_mut(&self.current_map_set).expect("Could not get current map set")
    }

    pub fn tiles(&self) -> Vec<TileId> {
        let mut tiles = Vec::with_capacity(500);
        for map_set in self.map_sets.values() {
            for map in &map_set.maps {
                for tile in &map.tiles {
                    if !tiles.contains(tile) {
                        tiles.push(*tile);
                    }        
                }
                for tile in &map.border.tiles {
                    if !tiles.contains(tile) {
                        tiles.push(*tile);
                    }
                }
            }
        }
        return tiles;
    }

}

impl World for WorldMapSetManager {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.map_sets.get(&self.current_map_set).map(|map_set| map_set.in_bounds(coords)).unwrap_or(false)
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.map_sets.get(&self.current_map_set).map(|map_set| map_set.tile(coords)).unwrap_or_default()
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        self.map_sets.get(&self.current_map_set).map(|map_set| map_set.walkable(coords)).unwrap_or(1)
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        self.map_sets.get(&self.current_map_set).map(|map_set| map_set.check_warp(coords)).unwrap_or_default()
    }

}