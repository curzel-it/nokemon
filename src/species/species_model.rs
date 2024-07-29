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
    pub capabilities: Vec<SpeciesCapability>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpeciesCapability {
    pub name: String,
    pub args: HashMap<String, String>
}

impl Species {
    pub fn default() -> Self {
        Self {
            id: "missingno".to_owned(),
            speed: 1.0,
            scale: 1.0,
            is_enemy: false,
            capabilities: vec![]
        }
    }
}

impl SpeciesCapability {
    pub fn get<T: FromStr>(&self, name: &str, fallback: T) -> T {
        if let Some(value) = self.args.get(name) {
            return value.parse().unwrap_or(fallback);
        }
        return fallback;
    }
}