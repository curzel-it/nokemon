use raylib::math::Rectangle;

use crate::{constants::{ASSETS_PATH, INFINITE_STOCK, TILE_SIZE}, entities::building::BuildingType, game_engine::keyboard_events_provider::KeyboardState, maps::{biome_tiles::Biome, constructions_tiles::Construction}};

#[derive(Debug)]
pub struct Inventory {
    pub is_open: bool,
    pub stock: Vec<InventoryItem>,
    pub selected_index: usize,
    sprite_sheet_path: String
}

#[derive(Debug)]
pub struct InventoryItem {
    pub item: Stockable,
    pub stock: i32
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            is_open: false,
            stock: vec![],
            selected_index: 0,
            sprite_sheet_path: format!("{}/inventory.png", ASSETS_PATH)
        }
    }

    pub fn update(&mut self, keyboard_state: &KeyboardState) {
        if keyboard_state.should_toggle_inventory() {
            self.is_open = !self.is_open;
        }
        if self.is_open {
            if keyboard_state.has_right_been_pressed && self.selected_index < self.stock.len() - 1 {
                self.selected_index += 1;
            }
            if keyboard_state.has_left_been_pressed && self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }
    }

    pub fn set_creative_mode(&mut self, is_enabled: bool) {        
        if is_enabled {
            self.stock = vec![];
            Stockable::all_possible_items().into_iter().for_each(|item| {
                self.stock.push(InventoryItem { item, stock: INFINITE_STOCK });
            });
            println!("Loaded up for creative mode: {:#?}", self.stock);
        }
    }

    pub fn sprite_sheet_path(&self) -> &str {
        &self.sprite_sheet_path
    }

    pub fn amount(&self, item: Stockable) -> i32 {
        if let Some(inventory_item) = self.stock.iter().find(|i| i.item == item) {
            return inventory_item.stock;
        }
        0
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Stockable {
    BiomeTile(Biome),
    ConstructionTile(Construction),    
    Building(BuildingType),    
}

impl Stockable {
    pub fn all_possible_items() -> Vec<Stockable> {
        vec![
            Stockable::BiomeTile(Biome::Water),
            Stockable::BiomeTile(Biome::Desert),
            Stockable::BiomeTile(Biome::Grass),
            Stockable::BiomeTile(Biome::Rock),
            Stockable::BiomeTile(Biome::Snow),
            Stockable::BiomeTile(Biome::LightWood),
            Stockable::BiomeTile(Biome::DarkWood),
            Stockable::ConstructionTile(Construction::WoodenFence),
            Stockable::Building(BuildingType::House),
        ]
    }

    pub fn texture_source_rect(&self) -> Rectangle {
        let (row, col) = self.texture_offsets();

        Rectangle {
            x: col as f32 * TILE_SIZE,
            y: row as f32 * TILE_SIZE,
            width: TILE_SIZE, 
            height: TILE_SIZE
        }
    }

    fn texture_offsets(&self) -> (u32, u32) {
         match self {
            Stockable::BiomeTile(biome) => match biome {
                Biome::Nothing => (0, 0),
                Biome::Water => (0, 1),
                Biome::Desert => (0, 2),
                Biome::Grass => (0, 3),
                Biome::Rock => (0, 4),
                Biome::Snow => (0, 5),
                Biome::LightWood => (0, 6),
                Biome::DarkWood => (0, 7),
            },
            Stockable::ConstructionTile(construction) => match construction {
                Construction::Nothing => (1, 0),
                Construction::WoodenFence => (1, 1),
            },
            Stockable::Building(building_type) => match building_type {
                BuildingType::House => (1, 2)
            }
        }
    }
}