use firecore_audio_lib::music::MusicId;
use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};

use ahash::AHashMap as HashMap;

use crate::MapSize;
use crate::MovementId;
use crate::TileId;

use crate::character::npc::NPC;
use crate::script::world::WorldScript;

use wild::WildEntry;
use warp::{WarpEntry, WarpDestination};

pub mod set;
pub mod chunk;
pub mod manager;

pub mod warp;
pub mod wild;
pub mod object;

pub trait World {

    // fn len(&self) -> usize;

    fn in_bounds(&self, coords: Coordinate) -> bool;

    fn tile(&self, coords: Coordinate) -> TileId;

    fn walkable(&self, coords: Coordinate) -> MovementId;

    fn check_warp(&self, coords: Coordinate) -> Option<warp::WarpDestination>;

}

#[derive(Default, Serialize, Deserialize)]
pub struct WorldMap {

    pub name: String,
    pub music: MusicId,

    pub width: MapSize,
    pub height: MapSize,

    pub tile_map: Vec<TileId>,
    pub border_blocks: [u16; 4],
    pub movement_map: Vec<MovementId>,

    pub warps: Vec<WarpEntry>,

    pub wild: Option<WildEntry>,
    
    pub npcs: HashMap<u8, NPC>,

    // pub objects: HashMap<u8, MapObject>,

    pub scripts: Vec<WorldScript>,

    #[serde(skip)]
    pub npc_active: Option<u8>,

}

impl World for WorldMap {

    // fn len(&self) -> usize {
    //     self.tile_map.len()
    // }

    fn in_bounds(&self, coords: Coordinate) -> bool {
        return !(coords.x < 0 || (coords.x as MapSize) >= self.width || coords.y < 0 || (coords.y as MapSize) >= self.height);
    }

    fn tile(&self, coords: Coordinate) -> TileId {
        self.tile_map[coords.x as usize + coords.y as usize * self.width as usize]
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        for npc in self.npcs.values() {
            if npc.position.coords == coords {
                return 1;
            }
        }
        // for object in self.objects.values() {
        //     if object.active {
        //         if object.location == coords {
        //             return 1;
        //         }
        //     }
        // }
        self.movement_map[coords.x as usize + coords.y as usize * self.width as usize]
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        for warp in &self.warps {
            if warp.location.in_bounds(&coords) {
                return Some(warp.destination.clone());
            }
        }
        return None;
    }

}