use raylib::math::Rectangle;

pub const FPS: u32 = 60;
pub const LOG_GAME_STATE: bool = false; // true;
pub const DEBUG_ENABLED: bool = true;
pub const BASE_ENTITY_SPEED: f32 = 30.0;
pub const SPECIES_PATH: &str = "/Users/curzel/dev/tower-defense/species";
pub const ASSETS_PATH: &str = "/Users/curzel/dev/tower-defense/assets";
pub const ANIMATIONS_FPS: f32 = 10.0;
pub const BASE_ENTITY_SIZE: f32 = 1.0;
pub const NO_PARENT: u32 = 0;

pub const ANIMATION_NAME_FRONT: &str = "front";
pub const MISSING_SPRITE: &str = "missing";
pub const ANIMATION_NAME_MOVEMENT_N: &str = "walk_e";
pub const ANIMATION_NAME_MOVEMENT_NE: &str = "walk_ne";
pub const ANIMATION_NAME_MOVEMENT_E: &str = "walk_e";
pub const ANIMATION_NAME_MOVEMENT_SE: &str = "walk_se";
pub const ANIMATION_NAME_MOVEMENT_S: &str = "walk_s";
pub const ANIMATION_NAME_MOVEMENT_SO: &str = "walk_so";
pub const ANIMATION_NAME_MOVEMENT_O: &str = "walk_o";
pub const ANIMATION_NAME_MOVEMENT_NO: &str = "walk_no";

// Test Stuff

pub const RECT_ORIGIN_FULL_HD: Rectangle = Rectangle::new(0.0, 0.0, 1920.0, 1080.0);
pub const RECT_ORIGIN_SQUARE_100: Rectangle = Rectangle::new(0.0, 0.0, 100.0, 100.0);
pub const RECT_X100_SQUARE_100: Rectangle = Rectangle::new(100.0, 0.0, 100.0, 100.0);
pub const RECT_ORIGIN_SQUARE_1000: Rectangle = Rectangle::new(0.0, 0.0, 1000.0, 1000.0);