use crate::{entities::{building::BuildingType, npc::NpcType}, maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::rect::Rect};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Stockable {
    BiomeTile(Biome),
    ConstructionTile(Construction),    
    Building(BuildingType),   
    Npc(NpcType), 
}

impl Stockable {
    pub fn all_possible_items() -> Vec<Stockable> {
        vec![
            Stockable::BiomeTile(Biome::Water),
            Stockable::BiomeTile(Biome::Desert),
            Stockable::BiomeTile(Biome::Grass),
            Stockable::BiomeTile(Biome::Rock),
            Stockable::BiomeTile(Biome::DarkRock),
            Stockable::BiomeTile(Biome::Snow),
            Stockable::BiomeTile(Biome::LightWood),
            Stockable::BiomeTile(Biome::DarkWood),
            Stockable::ConstructionTile(Construction::WoodenFence),
            Stockable::ConstructionTile(Construction::DarkRock),
            Stockable::ConstructionTile(Construction::LightWall),
            Stockable::Building(BuildingType::House),
            Stockable::Building(BuildingType::HouseTwoFloors),
            Stockable::Npc(NpcType::OldMan),
            Stockable::ConstructionTile(Construction::Nothing),
        ]
    }

    pub fn texture_source_rect(&self) -> Rect {
        let (row, col) = self.texture_offsets();
        Rect::new(col, row, 1, 1)
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
                Biome::DarkRock => (0, 8),
            },
            Stockable::ConstructionTile(construction) => match construction {
                Construction::Nothing => (1, 4),
                Construction::WoodenFence => (1, 1),
                Construction::DarkRock => (1, 5),
                Construction::LightWall => (1, 6),
            },
            Stockable::Building(building_type) => match building_type {
                BuildingType::House => (1, 2),
                BuildingType::HouseTwoFloors => (1, 7)
            },
            Stockable::Npc(npc_type) => match npc_type {
                NpcType::OldMan => (2, 1)
            }
        }
    }
}