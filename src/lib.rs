use serde::{Deserialize, Serialize};

pub mod map;
pub mod npc;
pub mod warp;
pub mod pokemon;
pub mod script;

pub type TileId = u16;
pub type MovementId = u8;
pub type MapSize = u16;

pub trait World {

    fn in_bounds(&self, x: isize, y: isize) -> bool;

    fn tile(&self, x: isize, y: isize) -> TileId;

    fn walkable(&self, x: isize, y: isize) -> MovementId;

    fn check_warp(&self, x: isize, y: isize) -> Option<warp::WarpEntry>;

}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BattleData {

    pub battle_type: BattleType,
    pub party: firecore_pokedex::pokemon::party::PokemonParty,
    pub trainer_data: Option<npc::trainer::TrainerData>,

}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BattleType { // move somewhere else

    Wild,
    Trainer,
    GymLeader,

}

impl Default for BattleType {
    fn default() -> Self {
        Self::Wild
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum BattleScreenTransitions {

    Flash,
    Trainer,

}

impl Default for BattleScreenTransitions {
    fn default() -> Self {
        Self::Flash
    }
}

pub fn test_move_code(move_code: u8, jump: bool) -> bool {
    move_code == 0x0C || move_code == 0x00 || move_code == 0x04 || jump
}