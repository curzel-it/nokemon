use raylib::math::Rectangle;

pub const FPS: u32 = 60;
pub const TILE_VARIATIONS_FPS: f32 = 1.0;
pub const ANIMATIONS_FPS: f32 = 10.0;
pub const LOG_GAME_STATE: bool = false; 
pub const BASE_ENTITY_SPEED: f32 = 30.0;
pub const ASSETS_PATH: &str = "/Users/curzel/dev/tower-defense/assets";
pub const WORLD_MAP_BIOME: &str = "/Users/curzel/dev/tower-defense/levels/world_biome.png";
pub const WORLD_MAP_CONSTRUCTIONS: &str = "/Users/curzel/dev/tower-defense/levels/world_constructions.png";
pub const NO_PARENT: u32 = 0;
pub const INFINITE_LIFESPAN: f32 = -420.0;
pub const INITIAL_CAMERA_VIEWPORT: Rectangle = Rectangle::new(0.0, 0.0, 1280.0, 720.0);
pub const TILE_SIZE: f32 = 16.0;
pub const TILE_TEXTURE_SIZE: f32 = 16.0;
pub const TILE_VARIATIONS_COUNT: u32 = 4;
pub const COLLISION_THRESHOLD: f32 = TILE_SIZE / 3.0;

pub const HERO_ENTITY_ID: u32 = 69;

// Test Stuff

pub const RECT_ORIGIN_FULL_HD: Rectangle = Rectangle::new(0.0, 0.0, 1920.0, 1080.0);
pub const RECT_ORIGIN_SQUARE_100: Rectangle = Rectangle::new(0.0, 0.0, 100.0, 100.0);
pub const RECT_X100_SQUARE_100: Rectangle = Rectangle::new(100.0, 0.0, 100.0, 100.0);
pub const RECT_ORIGIN_SQUARE_1000: Rectangle = Rectangle::new(0.0, 0.0, 1000.0, 1000.0);