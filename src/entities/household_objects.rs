use serde::{Deserialize, Serialize};

use super::{known_species::{SPECIES_BED, SPECIES_SEAT_BROWN, SPECIES_SEAT_GREEN, SPECIES_SEAT_ORANGE, SPECIES_SEAT_PINK, SPECIES_STAIRS_DOWN, SPECIES_STAIRS_UP, SPECIES_TABLE}, species::SpeciesConvertible};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdObject {
    StairsUp,
    StairsDown,
    SeatBrown,
    SeatGreen,
    SeatOrange,
    SeatPink,
    Table,
    Bed,
}

impl SpeciesConvertible for HouseholdObject {
    fn get_species_id(&self) -> u32 {
        match self {
            HouseholdObject::StairsUp => SPECIES_STAIRS_UP,
            HouseholdObject::StairsDown => SPECIES_STAIRS_DOWN,
            HouseholdObject::SeatBrown => SPECIES_SEAT_BROWN,
            HouseholdObject::SeatGreen => SPECIES_SEAT_GREEN,
            HouseholdObject::SeatOrange => SPECIES_SEAT_ORANGE,
            HouseholdObject::SeatPink => SPECIES_SEAT_PINK,
            HouseholdObject::Table => SPECIES_TABLE,
            HouseholdObject::Bed => SPECIES_BED,
        }
    }
}