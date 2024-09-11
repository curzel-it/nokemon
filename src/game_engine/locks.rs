use serde::{Deserialize, Serialize};

use crate::lang::localizable::LocalizableText;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

impl LockType {
    pub fn localized_name(&self) -> String {
        match self {
            LockType::None => "lock.name.none".localized(),
            LockType::Yellow => "lock.name.yellow".localized(),
            LockType::Red => "lock.name.red".localized(),
            LockType::Blue => "lock.name.blue".localized(),
            LockType::Green => "lock.name.green".localized(),
            LockType::Silver => "lock.name.silver".localized(),
        }
    }
}