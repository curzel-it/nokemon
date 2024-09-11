use serde::{Deserialize, Serialize};

use crate::{entities::{known_species::{SPECIES_KEY_BLUE, SPECIES_KEY_GREEN, SPECIES_KEY_RED, SPECIES_KEY_SILVER, SPECIES_KEY_YELLOW}, species::SpeciesId}, lang::localizable::LocalizableText};

const PRESSURE_PLATE_YELLOW: &str = "pressure_plate.yellow";
const PRESSURE_PLATE_RED: &str = "pressure_plate.red";
const PRESSURE_PLATE_BLUE: &str = "pressure_plate.blue";
const PRESSURE_PLATE_GREEN: &str = "pressure_plate.green";
const PRESSURE_PLATE_SILVER: &str = "pressure_plate.silver";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum LockType {
    #[default]
    None,
    Yellow,
    Red,
    Blue,
    Green,
    Silver
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

    pub fn key(&self) -> SpeciesId {
        match self {
            LockType::None => 0,
            LockType::Yellow => SPECIES_KEY_YELLOW,
            LockType::Red => SPECIES_KEY_RED,
            LockType::Blue => SPECIES_KEY_BLUE,
            LockType::Green => SPECIES_KEY_GREEN,
            LockType::Silver => SPECIES_KEY_SILVER,
        }
    }

    pub fn pressure_plate(&self) -> &str {
        match self {
            LockType::None => "",
            LockType::Yellow => PRESSURE_PLATE_YELLOW,
            LockType::Red => PRESSURE_PLATE_RED,
            LockType::Blue => PRESSURE_PLATE_BLUE,
            LockType::Green => PRESSURE_PLATE_GREEN,
            LockType::Silver => PRESSURE_PLATE_SILVER,
        }
    }
}