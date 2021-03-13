use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};
use firecore_util::music::Music;

use ahash::AHashMap as HashMap;

use crate::MapSize;
use crate::MovementId;
use crate::TileId;
use crate::World;
use crate::map_object::MapObject;
use crate::npc::NPC;
use crate::wild::WildEntry;
use crate::script::WorldScript;
use crate::warp::WarpEntry;

// pub mod manager;
pub mod set;
pub mod chunk;


#[derive(Default, Serialize, Deserialize)]
pub struct WorldMap {

    pub name: String,
    pub music: Music,

    pub width: MapSize,
    pub height: MapSize,

    pub tile_map: Vec<TileId>,
    pub border_blocks: [u16; 4],
    pub movement_map: Vec<MovementId>,

    pub fly_position: Coordinate,

    pub wild: Option<WildEntry>,
    pub warps: Vec<WarpEntry>,
    pub objects: HashMap<u8, MapObject>,
    pub npcs: Vec<NPC>,
    pub scripts: Vec<WorldScript>,

    #[serde(skip)]
    pub npc_active: Option<usize>,

    #[serde(skip)]
    pub script_npcs: HashMap<u8, NPC>,

}

impl World for WorldMap {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        return !(x < 0 || (x as u16) >= self.width || y < 0 || (y as u16) >= self.height);
    }

    fn tile(&self, x: isize, y: isize) -> TileId {
        self.tile_map[x as usize + y as usize * self.width as usize]
    }

    fn walkable(&self, x: isize, y: isize) -> MovementId {
        for npc in &self.npcs {
            if npc.position.coords.y == y && npc.position.coords.x == x {
                return 1;
            }
        }
        for npc in self.script_npcs.values() {
            if npc.position.coords.y == y && npc.position.coords.x == x {
                return 1;
            }
        }
        for object in self.objects.values() {
            if object.active {
                if object.location.y == y && object.location.x == x {
                    return 1;
                }
            }
        }
        self.movement_map[x as usize + y as usize * self.width as usize]
    }

    fn check_warp(&self, coords: &Coordinate) -> Option<WarpEntry> {
        for warp in &self.warps {
            if warp.location.in_bounds(&coords) {
                return Some(warp.clone());
            }
        }
        return None;
    }

}