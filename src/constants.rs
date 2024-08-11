use raylib::math::{Rectangle, Vector2};

pub const FPS: u32 = 60;
pub const LOG_GAME_STATE: bool = false; 
pub const BASE_ENTITY_SPEED: f32 = 30.0;
pub const ASSETS_PATH: &str = "/Users/curzel/dev/tower-defense/assets";
pub const BACKGROUND_TILE_GRASS: &str = "/Users/curzel/dev/tower-defense/assets/bg_tile_grass.png";
pub const WORLD_MAP_PATH: &str = "/Users/curzel/dev/tower-defense/assets/world_map-0.png";
pub const ANIMATIONS_FPS: f32 = 10.0;
pub const SCALE: f32 = 2.0;
pub const NO_PARENT: u32 = 0;
pub const INFINITE_LIFESPAN: f32 = -420.0;
pub const INITIAL_CAMERA_VIEWPORT: Rectangle = Rectangle::new(0.0, 0.0, 800.0, 600.0);
pub const GAME_SIZE: Vector2 = Vector2::new(1000.0, 1000.0);
pub const BG_TILE_SIZE: f32 = 42.0;
pub const BG_TILE_Z_INDEX: i32 = -1000;
pub const COLLISION_BOUNCE_FIX: f32 = 0.1;

pub const HERO_ENTITY_ID: u32 = 69;

pub const ANIMATION_NAME_FRONT: &str = "front";
pub const ANIMATION_NAME_MOVEMENT: &str = "walk";
pub const ANIMATION_NAME_STILL: &str = "still";
pub const MISSING_SPRITE: &str = "missing";

pub const DIRECTION_NAME_N: &str = "n";
pub const DIRECTION_NAME_NE: &str = "ne";
pub const DIRECTION_NAME_E: &str = "e";
pub const DIRECTION_NAME_SE: &str = "se";
pub const DIRECTION_NAME_S: &str = "s";
pub const DIRECTION_NAME_SW: &str = "sw";
pub const DIRECTION_NAME_W: &str = "w";
pub const DIRECTION_NAME_NW: &str = "nw";

// Test Stuff

pub const RECT_ORIGIN_FULL_HD: Rectangle = Rectangle::new(0.0, 0.0, 1920.0, 1080.0);
pub const RECT_ORIGIN_SQUARE_100: Rectangle = Rectangle::new(0.0, 0.0, 100.0, 100.0);
pub const RECT_X100_SQUARE_100: Rectangle = Rectangle::new(100.0, 0.0, 100.0, 100.0);
pub const RECT_ORIGIN_SQUARE_1000: Rectangle = Rectangle::new(0.0, 0.0, 1000.0, 1000.0);