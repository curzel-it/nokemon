use serde::Deserialize;
use std::string::String;

#[derive(Debug, Clone, Deserialize)]
pub struct Species {
    pub id: String,

    #[serde(default="df_zero")]
    pub speed: f32,

    #[serde(default="df_one")]
    pub width: f32,

    #[serde(default="df_one")]
    pub height: f32,

    #[serde(default="df_one_hundred")]
    pub hp: f32,

    #[serde(default="df_one_hundred")]
    pub dp: f32,

    #[serde(default="df_u32_one")]
    pub z_index: u32,

    #[serde(default="df_false")]
    pub is_enemy: bool,
    
    #[serde(default="df_false")]
    pub is_shooter: bool,
    
    #[serde(default="df_false")]
    pub is_bullet: bool,
    
    #[serde(default="df_true")]
    pub stays_inside_screen_bounds: bool,

    #[serde(default="df_one")]
    pub time_between_shots: f32,
}

impl Species {
    pub fn default() -> Self {
        Self {
            id: "missingno".to_owned(),
            speed: 1.0,
            width: 1.0,
            height: 1.0,
            hp: 100.0,
            dp: 100.0,
            z_index: 1,
            is_enemy: false,
            is_shooter: false,
            is_bullet: false,
            stays_inside_screen_bounds: true,
            time_between_shots: 1.0,
        }
    }
}

fn df_one() -> f32 {
    1.0
}

fn df_one_hundred() -> f32 {
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

fn df_true() -> bool {
    true
}