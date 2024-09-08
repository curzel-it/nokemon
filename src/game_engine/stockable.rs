use raylib::color::Color;

use crate::{constants::TILE_SIZE, entities::{building::BuildingType, household_objects::HouseholdObject, npc::NpcType}, maps::{biome_tiles::Biome, constructions_tiles::Construction}, texture, ui::components::{Spacing, View}, utils::{rect::Rect, vector::Vector2d}, zstack};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Stockable {
    BiomeTile(Biome),
    ConstructionTile(Construction),    
    Building(BuildingType),   
    Npc(NpcType), 
    HouseholdObject(HouseholdObject),
}

impl Stockable {
    pub fn texture_source_rect(&self) -> Rect {
        let (row, col) = self.texture_offsets();
        Rect::new(col, row, 1, 1)
    }

    fn texture_offsets(&self) -> (i32, i32) {
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
                Biome::DarkRock => (0, 8),
            },
            Stockable::ConstructionTile(construction) => match construction {
                Construction::Nothing => (1, 4),
                Construction::WoodenFence => (1, 1),
                Construction::DarkRock => (1, 5),
                Construction::LightWall => (1, 6),
            },
            Stockable::Building(building_type) => match building_type {
                BuildingType::House(variant) => (4, variant * 2 + 1),
                BuildingType::HouseTwoFloors(variant) => (4, variant * 2 + 2),
            },
            Stockable::Npc(npc_type) => match npc_type {
                NpcType::OldMan => (2, 2),
                NpcType::OldWoman => (2, 3),
            },
            Stockable::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => (3, 2),
                HouseholdObject::StairsDown => (3, 3),
                HouseholdObject::SeatBrown => (3, 4),
                HouseholdObject::SeatGreen => (3, 5),
                HouseholdObject::SeatOrange => (3, 6),
                HouseholdObject::SeatPink => (3, 7),
                HouseholdObject::Table => (3, 8),
                HouseholdObject::Bed => (3, 9),
            },
        }
    }
}

impl Stockable {
    pub fn ui(&self, sprite_sheet: u32, index: usize, selected_index: usize) -> View {
        let selected_size = 1.5 - 2.0 * Spacing::XS.unscaled_value() / TILE_SIZE;

        if index == selected_index {
            zstack!(
                Spacing::XS, 
                Color::YELLOW,
                texture!(
                    sprite_sheet, 
                    self.texture_source_rect(), 
                    Vector2d::new(selected_size, selected_size)
                )
            )
        } else {
            texture!(
                sprite_sheet, 
                self.texture_source_rect(), 
                Vector2d::new(1.5, 1.5)
            )
        }
    }
}