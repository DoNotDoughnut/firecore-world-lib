use firecore_util::BoundingBox;
use firecore_util::Destination;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpEntry {
    
    pub location: BoundingBox,

    pub destination: WarpDestination,

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpDestination {

    pub map_id: String,
    pub map_index: u16,

    pub position: Destination,
    #[serde(default)] // remove
    pub transition: WarpTransition,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpTransition {

    pub move_on_exit: bool,

    pub warp_on_tile: bool,

}

impl Default for WarpTransition {
    fn default() -> Self {
        Self {
            move_on_exit: false,
            warp_on_tile: true,
        }
    }
}