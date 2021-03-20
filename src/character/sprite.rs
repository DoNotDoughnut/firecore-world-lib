use serde::{Deserialize, Serialize};

// #[serde(try_from = "u8")]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SpriteIndexes {

    pub up: [u8; 4],
    pub down: [u8; 4],
    pub side: [u8; 4],

}

impl SpriteIndexes {

    pub const SPRITE_TYPE_STILL: SpriteIndexes = SpriteIndexes {
        up: [1; 4],
        down: [0; 4],
        side: [2; 4],
    };
    
    pub const SPRITE_TYPE_WALK: SpriteIndexes = SpriteIndexes {
        up: [1, 5, 1, 6],
        down: [0, 3, 0, 4],
        side: [2, 7, 2, 8],
    };

    pub const fn from_index(index: u8) -> &'static Self {
        match index {
            1 => &Self::SPRITE_TYPE_WALK,
            _ => &Self::SPRITE_TYPE_STILL,
        }
    }

}