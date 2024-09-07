use crate::{dialogues::tree::Dialogue, utils::rect::Rect};

// Fps
pub const FPS: u32 = 60;
pub const ANIMATIONS_FPS: f32 = 10.0;

// Default Props
pub const INITIAL_CAMERA_VIEWPORT: Rect = Rect::new(0, 0, 60, 40);
pub const WORLD_SIZE_ROWS: usize = 150;
pub const WORLD_SIZE_COLUMNS: usize = 200;
pub const INFINITE_LIFESPAN: f32 = -420.0;
pub const INFINITE_STOCK: i32 = -420;

// Input
pub const KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS_FIRST: f32 = 0.4;
pub const KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS: f32 = 0.1;

// Known entities
pub const HERO_ENTITY_ID: u32 = 420;

// Known locations
pub const WORLD_ID_NONE: u32 = 1000;
pub const WORLD_ID_DEMO: u32 = 1001;

// Animations
pub const WORLD_TRANSITION_TIME: f32 = 0.3;
pub const MENU_CLOSE_TIME: f32 = 0.2;
pub const MENU_OPEN_TIME: f32 = 0.1;

// Localization
pub const DEFAULT_LANG: &str = "en";

// Prefabs
pub const HOUSE_INTERIOR_ROWS: usize = 6;
pub const HOUSE_INTERIOR_COLUMNS: usize = 10;

// Paths
pub const ASSETS_PATH: &str = "assets";
pub const LEVELS_PATH: &str = "levels";
pub const DIALOGUES_PATH: &str = "levels/dialogues.json";
pub const FONT: &str = "fonts/PixelOperator/PixelOperator8.ttf";
pub const FONT_BOLD: &str = "fonts/PixelOperator/PixelOperator8-Bold.ttf";
pub const LOCALIZED_STRINGS_PATH: &str = "lang";

// Tiles
pub const TILE_VARIATIONS_FPS: f32 = 1.0;
pub const TILE_SIZE: f32 = 16.0;
pub const TILE_VARIATIONS_COUNT: i32 = 4;
pub const BASE_ENTITY_SPEED: f32 = TILE_SIZE * 2.0;
pub const STEP_INPUT_THRESHOLD: f32 = TILE_SIZE / 16.0;

// Sprite Sheets
pub const SPRITE_SHEET_BLANK: u32 = 1000;
pub const SPRITE_SHEET_INVENTORY: u32 = 1001;
pub const SPRITE_SHEET_BIOME_TILES: u32 = 1002;
pub const SPRITE_SHEET_CONSTRUCTION_TILES: u32 = 1003;
pub const SPRITE_SHEET_BUILDINGS: u32 = 1004;
pub const SPRITE_SHEET_BASE_ATTACK: u32 = 1005;
pub const SPRITE_SHEET_TELEPORTER: u32 = 1008;
pub const SPRITE_SHEET_HUMANOIDS: u32 = 1009;
pub const SPRITE_SHEET_HOUSEHOLD_OBJECTS: u32 = 1010;
pub const SPRITE_SHEET_MENU: u32 = 1011;