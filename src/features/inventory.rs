use std::collections::HashMap;

use common_macros::hash_map;
use raylib::ffi::Rectangle;

use crate::{constants::{ASSETS_PATH, TILE_SIZE}, maps::{biome_tiles::Biome, constructions_tiles::Construction}};

#[derive(Debug)]
pub struct Inventory {
    pub stock: HashMap<Stockable, u32>,
    creative_mode: bool,
    sprite_sheet_path: String
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            stock: hash_map!(),
            creative_mode: false,
            sprite_sheet_path: format!("{}/inventory.png", ASSETS_PATH)
        }
    }

    pub fn set_creative_mode(&mut self, is_enabled: bool) {
        self.creative_mode = is_enabled;
        
        if is_enabled {
            self.stock = hash_map!();
            Stockable::all_possible_items().into_iter().for_each(|item| {
                self.stock.insert(item, 1);
            });
            println!("Loaded up for creative mode: {:#?}", self.stock);
        }
    }

    pub fn sprite_sheet_path(&self) -> &str {
        &self.sprite_sheet_path
    }

    pub fn amount(&self, item: Stockable) -> u32 {
        if self.creative_mode {
            return 99;
        }
        if let Some(amount) = self.stock.get(&item) {
            return amount.clone()
        } else {
            return 0
        }
    }

    pub fn visible_items(&self) -> Vec<Stockable> {
        let mut items: Vec<Stockable> = self.stock
            .iter()
            .filter(|(_, amount)| { self.creative_mode || **amount > 0 })
            .map(|(item, _)| item.clone())
            .collect();
        items.sort_by_key(|item| item.texture_offsets());
        items
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Stockable {
    BiomeTile(Biome),
    ConstructionTile(Construction),    
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
            }
        }
    }
}