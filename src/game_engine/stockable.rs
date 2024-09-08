use raylib::color::Color;

use crate::{constants::{SPRITE_SHEET_INVENTORY, TILE_SIZE}, maps::{biome_tiles::Biome, constructions_tiles::Construction}, texture, ui::components::{Spacing, View}, utils::{rect::Rect, vector::Vector2d}, zstack};

use super::entity::{BuildingType, Species, HouseholdObject, NpcType, PickableObject};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Stockable {
    BiomeTile(Biome),
    ConstructionTile(Construction),    
    Building(BuildingType),   
    Npc(NpcType), 
    HouseholdObject(HouseholdObject),
    PickableObject(PickableObject),
}

impl Stockable {
    pub fn texture_source_rect(&self) -> Rect {
        let (y, x) = match self {
            Stockable::BiomeTile(biome) => match biome {
                Biome::Nothing => (0, 0),
                Biome::Water => (0, 1),
                Biome::Desert => (0, 2),
                Biome::Grass => (0, 3),
                Biome::Rock => (0, 4),
                Biome::Snow => (0, 5),
                Biome::LightWood => (0, 6),
                Biome::DarkWood => (0, 7),
                Biome::DarkRock => (0, 8),
            },
            Stockable::ConstructionTile(construction) => match construction {
                Construction::Nothing => (6, 1),
                Construction::WoodenFence => (1, 1),
                Construction::DarkRock => (1, 2),
                Construction::LightWall => (1, 3),
            },
            Stockable::Building(item) => Species::Building(*item).inventory_texture_offsets(),
            Stockable::Npc(item) => Species::Npc(*item).inventory_texture_offsets(),
            Stockable::HouseholdObject(item) => Species::HouseholdObject(*item).inventory_texture_offsets(),
            Stockable::PickableObject(item) => Species::PickableObject(*item).inventory_texture_offsets(),
        };
        Rect::new(x, y, 1, 1)
    }
}

impl Stockable {
    pub fn ui(&self, index: usize, selected_index: usize) -> View {
        let selected_size = 1.5 - 2.0 * Spacing::XS.unscaled_value() / TILE_SIZE;

        if index == selected_index {
            zstack!(
                Spacing::XS, 
                Color::YELLOW,
                texture!(
                    SPRITE_SHEET_INVENTORY, 
                    self.texture_source_rect(), 
                    Vector2d::new(selected_size, selected_size)
                )
            )
        } else {
            texture!(
                SPRITE_SHEET_INVENTORY, 
                self.texture_source_rect(), 
                Vector2d::new(1.5, 1.5)
            )
        }
    }
}