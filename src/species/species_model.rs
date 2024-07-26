use serde::Deserialize;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize)]
pub struct Species {
    pub id: String,
    pub speed: f32,
    pub scale: f32,
    pub capabilities: Vec<String>,
}

impl Species {
    pub fn has_capability(&self, name: &str) -> bool {
        self.capabilities.contains(&name.to_string())
    }

    pub fn default() -> Self {
        Self {
            id: "missingno".to_owned(),
            speed: 1.0,
            scale: 1.0,
            capabilities: vec![]
        }
    }
}