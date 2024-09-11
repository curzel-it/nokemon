use serde::{Deserialize, Serialize};

use crate::{dialogues::{models::{Dialogue, EntityDialogues}, repository::dialogue_by_id}, entities::species::{species_by_id, EntityType}, features::animated_sprite::AnimatedSprite, utils::{directions::Direction, rect::Rect, vector::Vector2d}};

use super::{locks::LockType, state_updates::{EngineStateUpdate, WorldStateUpdate}, storage::get_value_for_key, world::World};

#[derive(Debug, Copy, Clone)]
pub struct EntityProps {
    pub direction: Direction,
    pub frame: Rect,
    pub speed: f32,
    pub hittable_frame: Rect,
}

impl Default for EntityProps {
    fn default() -> Self {
        Self { 
            direction: Default::default(), 
            frame: Rect::square_from_origin(1), 
            speed: 0.0,
            hittable_frame: Rect::square_from_origin(1) 
        }
    }
}

pub type EntityId = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub frame: Rect,  
    pub name: String,  
    pub species_id: u32,  
    pub entity_type: EntityType,  
    pub offset: Vector2d,
    pub direction: Direction,
    pub current_speed: f32,
    pub is_rigid: bool,
    pub z_index: i32,
    pub sprite: AnimatedSprite,
    pub dialogues: EntityDialogues,
    pub time_immobilized: f32,
    pub destination: u32,

    #[serde(default)]
    pub lock_type: LockType,
}

impl Entity {
    pub fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {      
        let updates = match self.entity_type {
            EntityType::Hero => self.update_hero(world, time_since_last_update),
            EntityType::Npc => self.update_npc(world, time_since_last_update),
            EntityType::Building => self.update_building(world, time_since_last_update),
            EntityType::HouseholdObject => self.update_generic(world, time_since_last_update),
            EntityType::PickableObject => self.update_pickable_object(world, time_since_last_update),
            EntityType::Teleporter => self.update_teleporter(world, time_since_last_update),
        };
        
        self.sprite.update(time_since_last_update);  

        updates
    }

    pub fn sprite_sheet(&self) -> u32 {
        self.sprite.sheet_id
    }

    pub fn texture_source_rect(&self) -> Rect {
        self.sprite.texture_source_rect()
    }

    pub fn immobilize_for_seconds(&mut self, seconds: f32) {
        self.time_immobilized = seconds;
    }

    pub fn reset_speed(&mut self) {        
        self.current_speed = species_by_id(self.species_id).base_speed;
    }    
    
    pub fn center_in(&mut self, value: &Rect) {
        self.frame.center_in(value)
    }

    pub fn next_dialogue(&self) -> Option<Dialogue> {
        for option in &self.dialogues.options {
            if let Some(value) = get_value_for_key(&option.key) {
                if value == option.expected_value {
                    return dialogue_by_id(option.dialogue)
                }
            }
        }
        None
    }
}

impl Entity {
    fn update_generic(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {  
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        self.name.clone(), self.id, self.entity_type
                    )
                )
            ];   
        }
        vec![]
    }
}