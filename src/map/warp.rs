use crate::positions::{BoundingBox, Destination, Location};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type WarpId = tinystr::TinyStr16;
pub type Warps = HashMap<WarpId, WarpEntry>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarpEntry {
    pub location: BoundingBox,
    pub destination: WarpDestination,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarpDestination {
    pub location: Location,
    /// Where the player will end up
    pub destination: Destination,
}
