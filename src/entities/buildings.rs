use serde::{Deserialize, Serialize};

use super::{known_species::{SPECIES_HOUSE_1, SPECIES_HOUSE_2, SPECIES_HOUSE_3, SPECIES_HOUSE_TWO_FLOORS_1, SPECIES_HOUSE_TWO_FLOORS_2, SPECIES_HOUSE_TWO_FLOORS_3}, species::SpeciesConvertible};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    House(i32),
    HouseTwoFloors(i32),
}

impl SpeciesConvertible for BuildingType {
    fn get_species_id(&self) -> u32 {
        match self {
            BuildingType::House(variant) => match variant {
                0 => SPECIES_HOUSE_1,
                1 => SPECIES_HOUSE_2,
                _ => SPECIES_HOUSE_3,
            },
            BuildingType::HouseTwoFloors(variant) => match variant {
                0 => SPECIES_HOUSE_TWO_FLOORS_1,
                1 => SPECIES_HOUSE_TWO_FLOORS_2,
                _ => SPECIES_HOUSE_TWO_FLOORS_3,
            }
        }
    }
}