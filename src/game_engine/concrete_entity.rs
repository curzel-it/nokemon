use serde::{Deserialize, Serialize};

use crate::{dialogues::{models::{Dialogue, EntityDialogues}, repository::dialogue_by_id}, entities::models::EntityType, features::{animated_sprite::AnimatedSprite, keyboard_directions::set_direction_based_on_current_keys, linear_movement::move_linearly}, utils::{directions::Direction, rect::Rect, vector::Vector2d}};

use super::{entity::EntityProps, state_updates::{EngineStateUpdate, WorldStateUpdate}, storage::get_value_for_key, world::World};


#[derive(Debug, Serialize, Deserialize)]
pub struct ConcreteEntity {
    pub id: u32,
    pub frame: Rect,  
    pub name: String,  
    pub species: EntityType,  
    pub offset: Vector2d,
    pub direction: Direction,
    pub current_speed: f32,
    pub is_rigid: bool,
    pub z_index: i32,
    pub sprite: AnimatedSprite,
    pub dialogues: EntityDialogues,
    pub time_immobilized: f32,
}

impl ConcreteEntity {
    pub fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {      
        let updates = match self.species {
            EntityType::Hero => self.update_hero(world, time_since_last_update),
            EntityType::Npc(_) => self.update_npc(world, time_since_last_update),
            EntityType::Building(_) => self.update_generic(world, time_since_last_update),
            EntityType::HouseholdObject(_) => self.update_generic(world, time_since_last_update),
            EntityType::PickableObject(_) => self.update_pickable_object(world, time_since_last_update),
        };
        
        self.time_immobilized -= time_since_last_update;
        if self.time_immobilized <= 0.0 {
            self.move_linearly(world, time_since_last_update)
        }
        
        self.sprite.update(time_since_last_update);  

        updates
    }

    pub fn immobilize_for_seconds(&mut self, seconds: f32) {
        self.time_immobilized = seconds;
    }

    pub fn reset_speed(&mut self) {
        self.current_speed = self.species.base_speed();
    }

    fn next_dialogue(&self) -> Option<Dialogue> {
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

impl ConcreteEntity {
    fn update_hero(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {        
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        
        self.set_direction_based_on_current_keys(world.direction_based_on_current_keys);
        
        world_updates.push(self.cache_props());
        world_updates.push(self.move_camera_update());
        world_updates
    }

    fn cache_props(&self) -> WorldStateUpdate {
        WorldStateUpdate::CacheHeroProps(
            self.props()           
        )
    }

    fn props(&self) -> EntityProps {
        EntityProps {
            frame: self.frame,
            direction: self.direction,
            speed: self.current_speed,
            hittable_frame: Rect {
                x: self.frame.x,
                y: self.frame.y + 1,
                w: 1,
                h: 1,
            }
        }            
    }

    fn move_camera_update(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::CenterCamera(
                self.frame.x, 
                self.frame.y,
                self.offset
            )
        )
    }
}

impl ConcreteEntity {
    fn update_npc(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        if world.is_hero_around_and_on_collision_with(&self.frame) {
            if world.creative_mode {
                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowNpcOptions(
                            self.id, self.name.clone(), self.next_dialogue()
                        )
                    )
                ];  
            } else if let Some(dialogue) = self.next_dialogue() {
                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowDialogue(
                            self.id, self.name.clone(), dialogue,
                        )
                    )
                ];
            }             
        }  
        vec![]
    }
}

impl ConcreteEntity {
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

impl ConcreteEntity {
    fn update_pickable_object(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {        
        vec![]
    }
}