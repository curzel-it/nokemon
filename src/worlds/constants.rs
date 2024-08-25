use uuid::Uuid;

pub const WORLD_ID_NONE: Uuid = Uuid::from_bytes([0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0]);
pub const WORLD_ID_DEMO: Uuid = Uuid::from_bytes([0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,1]);