use serde::{Deserialize, Serialize};

pub static INDEX_TYPE_STILL: SpriteIndexes = SpriteIndexes {

    up: [1; 4],
    down: [0; 4],
    side: [2; 4],

};

pub static INDEX_TYPE_WALK: SpriteIndexes = SpriteIndexes {

    up: [1, 5, 1, 6],
    down: [0, 3, 0, 4],
    side: [2, 7, 2, 8],

};

// #[serde(try_from = "u8")]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SpriteIndexes {

    pub up: [u8; 4],
    pub down: [u8; 4],
    pub side: [u8; 4],

}

pub fn get_indexes(index: u8) -> &'static SpriteIndexes {
    match index {
        1 => &INDEX_TYPE_WALK,
        _ => &INDEX_TYPE_STILL,
    }
}

// impl TryFrom<u8> for SpriteIndexes {
//     type Error = MissingError;

//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//        match value {
//            0 => Ok(INDEX_TYPE_STILL),
//            1 => Ok(INDEX_TYPE_WALK),
//            _ => Err(MissingError::Missing),
//        }
//     }
// }

// impl Default for SpriteIndexes {
//     fn default() -> Self {
//         INDEX_TYPE_STILL
//     }
// }

// #[derive(Debug)]
// pub enum MissingError {
//     Missing,
// }

// impl std::error::Error for MissingError {}

// impl core::fmt::Display for MissingError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Could not deserialize from this number!")
//     }
// }