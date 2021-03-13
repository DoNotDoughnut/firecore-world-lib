use firecore_util::BoundingBox;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpEntry {
    
    pub location: BoundingBox,

    pub destination: WarpDestination, // world_id, map_set_id OR "world" for overworld map

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpDestination {

    // pub world_id: String,
    
    pub map_id: String,
    pub map_index: u16,

    pub x: isize,
    pub y: isize,

}