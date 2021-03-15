use firecore_util::Coordinate;

pub mod map;
pub mod wild;
pub mod warp;
pub mod map_object;
pub mod character;
pub mod script;
pub mod battle;

pub mod serialized;

pub type TileId = u16;
pub type MovementId = u8;
pub type MapSize = u16;

pub trait World {

    fn in_bounds(&self, coords: &Coordinate) -> bool;

    fn tile(&self, coords: &Coordinate) -> TileId;

    fn walkable(&self, coords: &Coordinate) -> MovementId;

    fn check_warp(&self, coords: &Coordinate) -> Option<warp::WarpEntry>;

}

pub fn test_move_code(move_code: u8, jump: bool) -> bool {
    move_code == 0x0C || move_code == 0x00 || move_code == 0x04 || jump
}