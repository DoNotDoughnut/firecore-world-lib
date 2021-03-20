use serde::{Deserialize, Serialize};

use firecore_audio_lib::music::MusicName;
use firecore_util::battle::BattleType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPCType {

    pub identifier: String,

    pub sprite_type: u8,

    pub trainer: Option<TrainerType>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrainerType {

    pub battle_type: BattleType,
    pub encounter_music: MusicName,

}