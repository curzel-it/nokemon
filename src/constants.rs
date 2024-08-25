use uuid::Uuid;

use crate::utils::rect::Rect;

pub const FPS: u32 = 60;
pub const ANIMATIONS_FPS: f32 = 10.0;
pub const BASE_ENTITY_SPEED: f32 = 30.0;
pub const INFINITE_LIFESPAN: f32 = -420.0;
pub const INITIAL_CAMERA_VIEWPORT: Rect = Rect::new(0, 0, 60, 40);
pub const COLLISION_THRESHOLD: f32 = TILE_SIZE / 3.0;
pub const INFINITE_STOCK: i32 = -420;

// Knonw entities
pub const NO_PARENT: Uuid = Uuid::from_bytes([0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,]);
pub const HERO_ENTITY_ID: Uuid = Uuid::from_bytes([0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,1,]);

// Paths
pub const ASSETS_PATH: &str = "assets";
pub const LEVELS_PATH: &str = "levels";
pub const FONT: &str = "fonts/PixelOperator/PixelOperator8.ttf";
pub const FONT_BOLD: &str = "fonts/PixelOperator/PixelOperator8-Bold.ttf";

// Tiles
pub const TILE_VARIATIONS_FPS: f32 = 1.0;
pub const TILE_SIZE: f32 = 16.0;
pub const TILE_VARIATIONS_COUNT: u32 = 4;

// Sprite Sheets
pub const SPRITE_SHEET_BLANK: u32 = 1000;
pub const SPRITE_SHEET_INVENTORY: u32 = 1001;
pub const SPRITE_SHEET_BIOME_TILES: u32 = 1002;
pub const SPRITE_SHEET_CONSTRUCTION_TILES: u32 = 1003;
pub const SPRITE_SHEET_BUILDINGS: u32 = 1004;
pub const SPRITE_SHEET_BASE_ATTACK: u32 = 1005;
pub const SPRITE_SHEET_TELEPORTER: u32 = 1008;
pub const SPRITE_SHEET_HUMANOIDS: u32 = 1009;
pub const SPRITE_SHEET_HUMANOIDS_COUNT: u32 = 2;