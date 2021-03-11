use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WarpEntry {
    
    pub x: isize, // change to bounding box
    pub y: isize,

    pub destination: WarpDestination, // world_id, map_set_id OR "world" for overworld map

}
#[derive(Clone, Serialize, Deserialize)]
pub struct WarpDestination {

    // pub world_id: String,
    
    pub map_id: String,
    pub map_index: u16,

    pub x: isize,
    pub y: isize,

}