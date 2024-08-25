use std::any::Any;

use crate::{constants::{HERO_ENTITY_ID, INFINITE_LIFESPAN}, features::{animated_sprite::AnimatedSprite, keyboard_directions::set_direction_according_to_keyboard_state, linear_movement::move_linearly}, game_engine::{entity::{Entity, EntityProps}, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, impl_humanoid_sprite_update, utils::{rect::Rect, vector::Vector2d}};

#[derive(Debug)]
pub struct Hero {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl Hero {
    pub fn new() -> Self {
        Self { 
            body: EntityBody {
                id: HERO_ENTITY_ID,
                frame: Rect::new(0, 0, 1, 2),
                offset: Vector2d::zero(),
                direction: Vector2d::zero(),
                current_speed: 3.0,
                base_speed: 3.0,
                creation_time: 0.0,
                is_rigid: true,
                z_index: 0,
                lifespan: INFINITE_LIFESPAN,
            },
            sprite: AnimatedSprite::new_humanoid(2),
        }
    }
}

impl_embodied_entity!(Hero);
impl_humanoid_sprite_update!(Hero);

impl Entity for Hero {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {        
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        set_direction_according_to_keyboard_state(self, &world.keyboard_state);
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
        world_updates.push(self.cache_props());
        world_updates.push(self.move_camera_update());
        world_updates
    }

    fn texture_source_rect(&self) -> Rect {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet(&self) -> u32 {
        self.sprite.sheet_id
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Hero {
    fn cache_props(&self) -> WorldStateUpdate {
        WorldStateUpdate::CacheHeroProps(
            self.props()           
        )
    }

    fn props(&self) -> EntityProps {
        EntityProps {
            frame: self.body.frame, 
            direction: self.body.direction
        }            
    }

    fn move_camera_update(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::CenterCamera(
                self.body.frame.x, 
                self.body.frame.y,
                self.body.offset
            )
        )
    }
}