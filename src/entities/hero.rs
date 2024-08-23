use crate::{constants::{HERO_ENTITY_ID, INFINITE_LIFESPAN, NO_PARENT, SPRITE_SHEET_HERO}, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, keyboard_directions::set_direction_according_to_keyboard_state, linear_movement::move_linearly, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::{Entity, EntityProps}, entity_body::{EmbodiedEntity, EntityBody}, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, impl_humanoid_sprite_update, impl_shooter, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

use super::surrounding_area_attack::SurroundingAreaAttack;

#[derive(Debug)]
pub struct Hero {
    body: EntityBody,
    time_to_next_shot: f32,
    time_between_shots: f32,
    sprite: AnimatedSprite,
}

impl Hero {
    pub fn new() -> Self {
        Self { 
            body: EntityBody {
                id: HERO_ENTITY_ID,
                parent_id: NO_PARENT,
                frame: Rect::new(0.0, 0.0, 19.0, 22.0),
                collision_insets: Insets::new(8.0, 1.0, 0.0, 1.0),
                direction: Vector2d::zero(),
                current_speed: 3.0,
                base_speed: 3.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: true,
                z_index: 0,
                is_ally: true,
                lifespan: INFINITE_LIFESPAN,
            },
            time_to_next_shot: 3.0,
            time_between_shots: 7.0,
            sprite: AnimatedSprite::new(
                SPRITE_SHEET_HERO, 
                3, 
                19, 
                22
            )
        }
    }
}

impl_embodied_entity!(Hero);
impl_humanoid_sprite_update!(Hero);
impl_shooter!(Hero, SurroundingAreaAttack);

impl Entity for Hero {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {        
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        set_direction_according_to_keyboard_state(self, &world.keyboard_state);
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut shoot_stuff(self, time_since_last_update));
        world_updates.append(&mut remove_automatically(self, world));
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
}

impl Hero {
    fn cache_props(&self) -> WorldStateUpdate {
        WorldStateUpdate::CacheHeroProps(
            EntityProps {
                frame: self.body.frame, 
                direction: self.body.direction, 
                speed: self.body.current_speed
            }            
        )
    }

    fn move_camera_update(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::CenterCamera(
                self.body.frame.x, self.body.frame.y
            )
        )
    }
}