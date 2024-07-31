use serde::Deserialize;
use std::string::String;

#[derive(Debug, Clone, Deserialize)]
pub struct Species {
    pub id: String,

    #[serde(default="df_zero")]
    pub speed: f32,

    #[serde(default="df_one")]
    pub scale: f32,

    #[serde(default="df_u32_one")]
    pub z_index: u32,

    #[serde(default="df_false")]
    pub is_enemy: bool,
    
    #[serde(default)]
    pub is_shooter: bool,

    #[serde(default="df_one")]
    pub bullets_per_minute: f32,
}

impl Species {
    pub fn default() -> Self {
        Self {
            id: "missingno".to_owned(),
            speed: 1.0,
            scale: 1.0,
            z_index: 1,
            is_enemy: false,
            is_shooter: false,
            bullets_per_minute: 1.0,
        }
    }
}

fn df_one() -> f32 {
    1.0
}

fn df_u32_one() -> u32 {
    1
}

fn df_zero() -> f32 {
    0.0
}

fn df_false() -> bool {
    false
}