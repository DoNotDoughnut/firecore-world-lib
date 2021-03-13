use firecore_util::Coordinate;
use serde::{Serialize, Deserialize};
use ahash::AHashMap as HashMap;

use crate::World;
use crate::warp::WarpEntry;

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
        self.map_sets.get(&self.current_map_set).expect("Could not get current map set")
    }

    pub fn map_set_mut(&mut self) -> &mut WorldMapSet {
        self.map_sets.get_mut(&self.current_map_set).expect("Could not get current map set")
    }

    pub fn tiles(&self) -> Vec<crate::TileId> {
        let mut tiles = Vec::new();
        for map_set in self.map_sets.values() {
            for map in &map_set.maps {
                for tile_id in &map.tile_map {
                    if !tiles.contains(tile_id) {
                        tiles.push(*tile_id);
                    }        
                }
                for tile_id in &map.border_blocks {
                    if !tiles.contains(tile_id) {
                        tiles.push(*tile_id);
                    }
                }
            }
        }
        return tiles;
    }

}

impl World for WorldMapSetManager {

    fn in_bounds(&self, coords: &Coordinate) -> bool {
        self.map_set().in_bounds(coords)
    }

    fn tile(&self, coords: &Coordinate) -> u16 {
        self.map_set().tile(coords)
    }

    fn walkable(&self, coords: &Coordinate) -> u8 {
        self.map_set().walkable(coords)
    }

    fn check_warp(&self, coords: &Coordinate) -> Option<WarpEntry> {
        self.map_set().check_warp(coords)
    }

}