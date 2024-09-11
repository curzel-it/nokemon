use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LockType {
    None,
    Yellow,
    Red,
    Blue,
    Green,
    Silver
}

impl Default for LockType {
    fn default() -> Self {
        LockType::None
    }
}