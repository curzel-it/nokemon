use raylib::math::Rectangle;

pub const FPS: u32 = 60;
pub const DEBUG_ENABLED: bool = true;
pub const BASE_ENTITY_SPEED: f32 = 30.0;
pub const SPECIES_PATH: &str = "/Users/curzel/dev/tower-defense/species";
pub const ASSETS_PATH: &str = "/Users/curzel/dev/tower-defense/assets";
pub const ANIMATIONS_FPS: f32 = 10.0;
pub const BASE_ENTITY_SIZE: f32 = 50.0;

pub const GAME_ENTITY_ID: u32 = 0;

pub const ANIMATION_NAME_MOVEMENT: &str = "walk";
pub const ANIMATION_NAME_FRONT: &str = "front";
pub const MISSING_SPRITE: &str = "missing";

// Test Stuff

pub const RECT_ORIGIN_FULL_HD: Rectangle = Rectangle::new(0.0, 0.0, 1920.0, 1080.0);
pub const RECT_ORIGIN_SQUARE_100: Rectangle = Rectangle::new(0.0, 0.0, 100.0, 100.0);
pub const RECT_X100_SQUARE_100: Rectangle = Rectangle::new(100.0, 0.0, 100.0, 100.0);
pub const RECT_ORIGIN_SQUARE_1000: Rectangle = Rectangle::new(0.0, 0.0, 1000.0, 1000.0);