use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};
use firecore_audio::music::Music;

use ahash::AHashMap as HashMap;

use crate::MapSize;
use crate::MovementId;
use crate::TileId;
use crate::World;
use crate::npc::NPC;
use crate::pokemon::WildEntry;
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
    // pub draw_color: Option<macroquad::prelude::Color>,

    pub wild: Option<WildEntry>,

    pub warps: Vec<WarpEntry>,
    pub npcs: Vec<NPC>,
    
    #[serde(skip)]
    pub npc_active: Option<usize>,

    // Scripts

    pub scripts: Vec<WorldScript>,

    #[serde(skip)]
    pub script_npcs: HashMap<u8, NPC>,
    // pub script_manager: MapScriptManager,

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
        self.movement_map[x as usize + y as usize * self.width as usize]
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        for warp in &self.warps {
            if warp.x == x {
                if warp.y == y {
                    return Some(warp.clone());
                }
            }
        }
        return None;
    }

}