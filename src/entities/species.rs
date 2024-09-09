use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;
use crate::constants::{HERO_ENTITY_ID, SPECIES_PATH, SPRITE_SHEET_BIOME_TILES};
use crate::dialogues::models::EntityDialogues;
use crate::features::animated_sprite::AnimatedSprite;
use crate::game_engine::entity::Entity;
use crate::lang::localizable::LocalizableText;
use crate::utils::directions::Direction;
use crate::utils::ids::get_next_id;
use crate::utils::rect::Rect;
use crate::utils::vector::Vector2d;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub id: u32,
    pub name: String,
    pub entity_type: EntityType,
    pub z_index: i32,
    pub base_speed: f32,
    pub is_rigid: bool,
    pub inventory_texture_offset: (i32, i32),
    pub sprite_frame: Rect,
    pub sprite_sheet_id: u32,
    pub sprite_number_of_frames: i32,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Hero,
    Building,   
    Npc, 
    HouseholdObject,
    PickableObject,
    Teleporter,
}

impl Species {
    pub fn make_entity(&self) -> Entity {
        Entity {
            id: self.next_entity_id(),
            frame: self.sprite_frame,  
            species_id: self.id,  
            entity_type: self.entity_type,
            offset: Vector2d::zero(),
            direction: Direction::Unknown,
            current_speed: 0.0,
            is_rigid: self.is_rigid,
            z_index: self.z_index,
            sprite: self.make_sprite(false),
            dialogues: EntityDialogues::empty(),
            time_immobilized: 0.0,
            name: self.name.localized(),
            destination: 0,
        }
    }
    
    fn make_sprite(&self, _: bool) -> AnimatedSprite {
        AnimatedSprite::new(
            self.sprite_sheet_id,
            self.sprite_frame,
            self.sprite_number_of_frames
        )
    }

    fn next_entity_id(&self) -> u32 {
        match self.entity_type {
            EntityType::Hero => HERO_ENTITY_ID,
            _ => get_next_id()
        }
    }
}

lazy_static! {
    pub static ref ALL_SPECIES: Vec<Species> = {
        let mut file = File::open(SPECIES_PATH).expect("Could not open species_data.json");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Could not read species_data.json");
        serde_json::from_str(&data).expect("Error parsing species_data.json")
    };
}

const SPECIES_NONE: Species = Species {
    id: 0,
    name: String::new(),
    entity_type: EntityType::Npc,
    z_index: 1000,
    base_speed: 0.0,
    is_rigid: false,
    inventory_texture_offset: (0, 0),
    sprite_frame: Rect::new(0, 0, 0, 0),
    sprite_sheet_id: SPRITE_SHEET_BIOME_TILES,
    sprite_number_of_frames: 1,
};

pub fn species_by_id(species_id: u32) -> Species {
    ALL_SPECIES.iter().find(|s| s.id == species_id).cloned().unwrap_or(SPECIES_NONE)
}

pub fn make_entity_by_species(species_id: u32) -> Entity {
    species_by_id(species_id).make_entity()
}