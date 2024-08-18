use std::collections::HashMap;

use common_macros::hash_map;

use crate::maps::{biome_tiles::Biome, constructions_tiles::Construction};

pub struct Inventory {
    pub stock: HashMap<Stockable, u32>
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            stock: hash_map!(),
        }
    }

    pub fn amount(&self, item: Stockable) -> u32 {
        return if let Some(amount) = self.stock.get(&item) {
            amount.clone()
        } else {
            0
        }
    }

    pub fn visible_items(&self) -> Vec<Stockable> {
        let mut items: Vec<Stockable> = self.stock
            .iter()
            .filter(|(_, amount)| { **amount > 0 })
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
    fn texture_offsets(&self) -> (u32, u32) {
         match self {
            Stockable::BiomeTile(biome) => match biome {
                Biome::Nothing => (0, 0),
                Biome::Water => (0, 1),
                Biome::Desert => (0, 2),
                Biome::Grass => (0, 3),
                Biome::Rock => (0, 4),
                Biome::Snow => (0, 5),
            },
            Stockable::ConstructionTile(construction) => match construction {
                Construction::Nothing => (1, 0),
                Construction::WoodenFence => (1, 1),
                Construction::House => (1, 2),
            }
        }
    }
}