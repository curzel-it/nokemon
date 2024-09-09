use serde::{Deserialize, Serialize};

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