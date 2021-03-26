use std::u8;

use serde::{Deserialize, Serialize};

use firecore_util::Direction;

use crate::character::Character;
use crate::character::player::PlayerCharacter;

use super::World;
use super::chunk::map::WorldChunkMap;
use super::set::manager::WorldMapSetManager;
use super::warp::WarpDestination;

#[derive(Default, Deserialize, Serialize)]
pub struct WorldMapManager {

    pub chunk_map: WorldChunkMap,
    pub map_set_manager: WorldMapSetManager,

    #[serde(skip)]
    pub chunk_active: bool,

    #[serde(skip)]
    pub player: PlayerCharacter,

    #[serde(skip)]
    pub warp: Option<WarpDestination>,

}

impl WorldMapManager {

    // pub fn get_map_music(&self) -> MusicId {
    //     if self.chunk_active {
    //         self.chunk_map.current_chunk().map.music
    //     } else {
    //         self.map_set_manager.map_set().map().music
    //     }
    // }

    pub fn try_move(&mut self, direction: Direction, delta: f32) -> bool { // return boolean to update music

        let mut update = false;

        self.player.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = self.player.position.local.coords + offset;

        let in_bounds = if self.chunk_active {
            self.chunk_map.in_bounds(coords)
        } else {
            self.map_set_manager.in_bounds(coords)
        };

        let move_code = if self.chunk_active {
            if in_bounds {
                self.chunk_map.walkable(coords)
            } else {
               let (code, do_update) = self.chunk_map.walk_connections(&mut self.player.position, coords);
               update = do_update;
               code
            }
        } else {
            if in_bounds {
                self.map_set_manager.walkable(coords)
            } else {
                1
            }
        };

        let allow = if self.chunk_active {
            if let Some(destination) = self.chunk_map.check_warp(coords) {
                if !destination.transition.warp_on_tile {
                    self.warp = Some(destination);
                    return true;
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            if let Some(destination) = self.map_set_manager.check_warp(coords) {
                if !destination.transition.warp_on_tile {
                    self.warp = Some(destination);
                    return true;
                } else {
                    true
                }
            } else {
                false
            }
        } || if let Some(tile_id) = if self.chunk_active {
            self.chunk_map.tile(coords)
        } else {
            self.map_set_manager.tile(coords)
        } {
            match direction {
                Direction::Up => false,
                Direction::Down => match tile_id  {
                    135 | 176 | 177 | 143 | 151 | 184 | 185 | 192 | 193 | 217 | 1234 => true,
                    _ => false,
                },
                Direction::Left => tile_id == 133,
                Direction::Right => tile_id == 134,
            }
        } else {
            false
        };

        if can_move(move_code) || allow || self.player.properties.noclip {
            let mult = self.player.properties.speed * 60.0 * delta;
            self.player.position.local.offset = direction.pixel_offset().scale(mult);
        }

        update
    }

    pub fn update_chunk(&mut self, index: u16) {
        self.chunk_map.current_chunk = index;
    }

    pub fn update_map_set(&mut self, id: &String, index: u16) {
        self.map_set_manager.set(id);
        self.map_set_manager.set_index(index as usize);
    }

    pub fn warp(&mut self, destination: WarpDestination) {
        if destination.map_id.as_str().eq("world") {
            self.warp_to_chunk_map(&destination);
        } else {
            self.warp_to_map_set(&destination);
        }
    }

    pub fn warp_to_chunk_map(&mut self, destination: &WarpDestination) {
        if !self.chunk_active {
            self.chunk_active = true;
        }
        if let Some(chunk) = self.chunk_map.update_chunk(&destination.map_index) {
            self.player.position.global = chunk.coords;
            self.player.position.local.from_destination(destination.position);
        }
    }

    pub fn warp_to_map_set(&mut self, destination: &WarpDestination) {
        if self.chunk_active {
            self.chunk_active = false;
        }
        self.update_map_set(&destination.map_id, destination.map_index);
        self.player.position.global = firecore_util::Coordinate { x: 0, y: 0 };
        self.player.position.local.from_destination(destination.position);
    }

}

pub fn can_move(move_code: u8) -> bool {
    move_code == 0x0C | 0x00 | 0x04
}