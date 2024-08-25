use uuid::Uuid;

pub const WORLDS_PATH: &str = "/Users/curzel/dev/tower-defense/worlds/";
pub const WORLD_ID_NONE: Uuid = Uuid::from_bytes([0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0]);
pub const WORLD_ID_DEMO: Uuid = Uuid::from_bytes([0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,1]);