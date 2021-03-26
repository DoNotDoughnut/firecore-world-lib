use firecore_util::Coordinate;
use firecore_util::GlobalPosition;
use serde::{Deserialize, Serialize};
use ahash::AHashMap as HashMap;

use crate::MovementId;
use crate::TileId;

use crate::map::World;
use crate::map::warp::WarpDestination;

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

    pub fn change_chunk(&mut self, chunk_index: u16, player_pos: &mut GlobalPosition) {
        if let Some(chunk) = self.update_chunk(&chunk_index) {
            {
                player_pos.local.coords = player_pos.absolute() - chunk.coords;
                player_pos.global = chunk.coords;
            }            
        }
        
    }

    pub fn chunk_at(&self, coords: Coordinate) -> Option<(&u16, &WorldChunk)> {
        for chunk in &self.chunks {
            if chunk.1.in_bounds(coords) {
                return Some(chunk);
            }
        }
        None
    }

    pub fn chunk_id_at(&self, coords: Coordinate) -> Option<u16> {
        for (id, chunk) in &self.chunks {
            if chunk.in_bounds(coords - chunk.coords) {
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
        let mut tiles = Vec::with_capacity(1000);
        for chunk in self.chunks.values() {
            for tile in &chunk.map.tiles {
                if !tiles.contains(tile) {
                    tiles.push(*tile);
                }
            }            
            for tile in &chunk.map.border.tiles {
                if !tiles.contains(tile) {
                    tiles.push(*tile);
                }
            }
        }
        return tiles;
    }

    pub fn walk_connections(&mut self, player_pos: &mut GlobalPosition, coords: Coordinate) -> (MovementId, bool) {
        let absolute = player_pos.global + coords;
        let mut move_code = 1;
        let mut chunk = None;
        for (index, connection) in self.connections() {
            let connection_coords = absolute - connection.coords;
            if connection.in_bounds(connection_coords) {
                move_code = connection.walkable(connection_coords);
                chunk = Some(*index);
            }
        }
        if let Some(chunk) = chunk {
            if crate::map::manager::can_move(move_code) {
                self.change_chunk(chunk, player_pos);
                return (move_code, true);
            }
        }
        (move_code, false)
    }

}

impl World for WorldChunkMap {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.current_chunk().in_bounds(coords)
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        let current = self.current_chunk();
        match current.tile(coords) {
            Some(tile) => Some(tile),
            None => {
                for connection in &current.connections {
                    if let Some(connection) = self.chunks.get(connection) {
                        let tile = connection.tile(coords);
                        if tile.is_some() {
                            return tile;
                        }
                    }
                }
                None
            }
        }
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        self.current_chunk().walkable(coords)  
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        self.current_chunk().check_warp(coords)
    }
    
}