use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize)]
pub struct Species {
    pub id: String,
    pub speed: f32,
    pub scale: f32,
    pub is_enemy: bool,
    pub is_shooter: bool,
    pub time_between_shots: f32,
    pub time_to_next_shot: f32,
}

impl Species {
    pub fn default() -> Self {
        Self {
            id: "missingno".to_owned(),
            speed: 1.0,
            scale: 1.0,
            is_enemy: false,
            is_shooter: false,
            time_between_shots: 0.0,
            time_to_next_shot: 0.0
        }
    }
}