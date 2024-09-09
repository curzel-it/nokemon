use serde::{Deserialize, Serialize};

use crate::{constants::{HERO_ENTITY_ID, SPRITE_SHEET_ANIMATED_OBJECTS, SPRITE_SHEET_BUILDINGS, SPRITE_SHEET_HOUSEHOLD_OBJECTS, SPRITE_SHEET_HUMANOIDS, SPRITE_SHEET_TELEPORTER, WORLD_ID_NONE}, dialogues::{models::{Dialogue, EntityDialogues}, repository::dialogue_by_id}, entities::species::Species, features::animated_sprite::AnimatedSprite, lang::localizable::LocalizableText, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

use super::{state_updates::{EngineStateUpdate, WorldStateUpdate}, storage::get_value_for_key, world::World};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub frame: Rect,  
    pub name: String,  
    pub species: Species,  
    pub offset: Vector2d,
    pub direction: Direction,
    pub current_speed: f32,
    pub is_rigid: bool,
    pub z_index: i32,
    pub sprite: AnimatedSprite,
    pub dialogues: EntityDialogues,
    pub time_immobilized: f32,
    pub destination: u32,
}

impl Entity {
    pub fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {      
        let updates = match self.species {
            Species::Hero => self.update_hero(world, time_since_last_update),
            Species::Npc(_) => self.update_npc(world, time_since_last_update),
            Species::Building(_) => self.update_generic(world, time_since_last_update),
            Species::HouseholdObject(_) => self.update_generic(world, time_since_last_update),
            Species::PickableObject(_) => self.update_pickable_object(world, time_since_last_update),
            Species::Teleporter => self.update_teleporter(world, time_since_last_update),
        };
        
        self.time_immobilized -= time_since_last_update;
        if self.time_immobilized <= 0.0 {
            self.move_linearly(world, time_since_last_update)
        }
        
        self.sprite.update(time_since_last_update);  

        updates
    }

    pub fn sprite_sheet(&self) -> u32 {
        self.species.sprite_sheet()
    }

    pub fn texture_source_rect(&self) -> Rect {
        self.sprite.texture_source_rect()
    }

    pub fn immobilize_for_seconds(&mut self, seconds: f32) {
        self.time_immobilized = seconds;
    }

    pub fn reset_speed(&mut self) {
        self.current_speed = self.species.base_speed();
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
                        self.id
                    )
                )
            ];   
        }
        vec![]
    }
}