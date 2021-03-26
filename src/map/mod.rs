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
// pub mod object;

pub trait World {

    // fn len(&self) -> usize;

    fn in_bounds(&self, coords: Coordinate) -> bool;

    fn tile(&self, coords: Coordinate) -> Option<TileId>;

    fn walkable(&self, coords: Coordinate) -> MovementId; // not an option because can return 1

    fn check_warp(&self, coords: Coordinate) -> Option<warp::WarpDestination>;

}

#[derive(Default, Serialize, Deserialize)]
pub struct WorldMap {

    pub name: String,
    pub music: MusicId,

    pub width: MapSize,
    pub height: MapSize,

    pub tiles: Vec<TileId>,
    pub movements: Vec<MovementId>,

    pub border: Border, // border blocks

    // Map objects

    pub warps: Vec<WarpEntry>,

    pub wild: Option<WildEntry>,
    
    pub npcs: HashMap<u8, NPC>,

    // pub objects: HashMap<u8, MapObject>,

    pub scripts: Vec<WorldScript>,

    // Map-specific runtime stuff

    #[serde(skip)]
    pub npc_active: Option<u8>,

}

impl WorldMap {

    pub fn tile_or_panic(&self, x: usize, y: usize) -> TileId {
        self.tiles[x + y * self.width]
    }

}

impl World for WorldMap {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        return !(coords.x < 0 || coords.x >= self.width as isize || coords.y < 0 || coords.y >= self.height as isize);
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        if self.in_bounds(coords) {
            Some(self.tiles[coords.x as usize + coords.y as usize * self.width])
        } else {
            None
        }        
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
        self.movements[coords.x as usize + coords.y as usize * self.width]
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        for warp in &self.warps {
            if warp.location.in_bounds(&coords) {
                return Some(warp.destination.clone());
            }
        }
        None
    }

}

#[derive(Default, Serialize, Deserialize)]
pub struct Border {

    pub tiles: Vec<TileId>,
    pub size: u8, // length or width (border is a square)

}