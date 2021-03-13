use firecore_util::Coordinate;
use firecore_util::GlobalPosition;
use serde::{Deserialize, Serialize};
use ahash::AHashMap as HashMap;

use crate::MovementId;
use crate::TileId;
use crate::World;
use crate::test_move_code;
use crate::warp::WarpEntry;

use super::WorldChunk;

#[derive(Default, Deserialize, Serialize)]
pub struct WorldChunkMap {

    pub chunks: HashMap<u16, WorldChunk>,

    #[serde(skip)]
    pub current_chunk: u16,

}

impl WorldChunkMap {

    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            current_chunk: 2,
        }
    }

    pub fn update_chunk(&mut self, chunk_id: &u16) -> Option<&WorldChunk> {
        if let Some(chunk) = self.chunks.get(chunk_id) {
            self.current_chunk = *chunk_id;
            return Some(chunk);
        } else {
            return None;
        }
    }

    pub fn change_chunk(&mut self, chunk: u16, player_pos: &mut GlobalPosition) {
        if let Some(chunk1) = self.update_chunk(&chunk) {
            {
                player_pos.local.coords.x = player_pos.get_x() - chunk1.x;
                player_pos.local.coords.y = player_pos.get_y() - chunk1.y;
                player_pos.offset.x = chunk1.x;
                player_pos.offset.y = chunk1.y;
            }            
        }
        
    }

    pub fn chunk_at(&self, coords: &Coordinate) -> Option<(&u16, &WorldChunk)> {
        for chunk in &self.chunks {
            if chunk.1.in_bounds(coords) {
                return Some(chunk);
            }
        }
        None
    }

    pub fn chunk_id_at(&self, coords: &Coordinate) -> Option<u16> {
        for (id, chunk) in &self.chunks {
            if chunk.in_bounds(&coords.subtract(chunk.x, chunk.y)) {
                return Some(*id);
            }
        }
        None
    }

    pub fn current_chunk(&self) -> &WorldChunk {
        self.chunks.get(&self.current_chunk).expect("Could not get current chunk")
    }

    pub fn current_chunk_mut(&mut self) -> &mut WorldChunk {
        self.chunks.get_mut(&self.current_chunk).expect("Could not get current chunk")
    }

    pub fn connections(&self) -> Vec<(&u16, &WorldChunk)> {
        self.current_chunk().connections.iter().map(|connection| (connection, self.chunks.get(connection).expect("Could not get connected chunks"))).collect()
    }

    pub fn tiles(&self) -> Vec<crate::TileId> {
        let mut tiles = Vec::new();
        for chunk in self.chunks.values() {
            for tile_id in &chunk.map.tile_map {
                if !tiles.contains(tile_id) {
                    tiles.push(*tile_id);
                }
            }
            for tile_id in &chunk.map.border_blocks {
                if !tiles.contains(tile_id) {
                    tiles.push(*tile_id);
                }
            }
        }
        return tiles;
    }

    pub fn walk_connections(&mut self, player_pos: &mut GlobalPosition, coords: &Coordinate) -> MovementId {
        let mut move_code = 1;
        let mut chunk = None;
        for connection in self.connections() {
            let coords = coords.subtract(connection.1.x, connection.1.y);
            if connection.1.in_bounds(&coords) {
                move_code = connection.1.walkable(&coords);
                chunk = Some(*connection.0);
            }
        }
        if let Some(chunk) = chunk {
            if test_move_code(move_code, false) {
                self.change_chunk(chunk, player_pos);   
            }
        }
        return move_code;
    }

}

impl World for WorldChunkMap {

    fn in_bounds(&self, coords: &Coordinate) -> bool {
        self.current_chunk().in_bounds(coords)
    }

    fn tile(&self, coords: &Coordinate) -> TileId {
        if let Some(tile) = self.current_chunk().safe_tile(coords) {
            return tile;
        } else {
            let current_chunk = self.current_chunk();
            for connection in &current_chunk.connections {
                let chunk = self.chunks.get(connection).expect("Could not get current chunk");
                if let Some(tile) = chunk.safe_tile(coords) {
                    return tile;
                }
            }
            if coords.y % 2 == 0 {
                if coords.x % 2 == 0 {
                    current_chunk.map.border_blocks[0]
                } else {
                    current_chunk.map.border_blocks[2]
                }
            } else {
                if coords.x % 2 == 0 {
                    current_chunk.map.border_blocks[1]
                } else {
                    current_chunk.map.border_blocks[3]
                }
            }
        }
    }

    fn walkable(&self, coords: &Coordinate) -> MovementId {
        self.current_chunk().walkable(coords)  
    }

    fn check_warp(&self, coords: &Coordinate) -> Option<WarpEntry> {
        self.current_chunk().check_warp(coords)
    }
    
}