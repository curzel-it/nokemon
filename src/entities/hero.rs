use raylib::math::{Rectangle, Vector2};

use crate::{constants::{HERO_ENTITY_ID, INFINITE_LIFESPAN, NO_PARENT}, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, keyboard_directions::set_direction_according_to_keyboard_state, linear_movement::move_linearly, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, world::World, state_updates::WorldStateUpdate}, impl_embodied_entity, impl_humanoid_sprite_update, impl_shooter, utils::geometry_utils::Insets};

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
                frame: Rectangle::new(0.0, 0.0, 19.0, 22.0),
                collision_insets: Insets::new(8.0, 1.0, 0.0, 1.0),
                direction: Vector2::zero(),
                current_speed: 3.0,
                base_speed: 3.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: true,
                z_index: 0,
                is_ally: true,
                is_bullet: false,
                lifespan: INFINITE_LIFESPAN,
            },
            time_to_next_shot: 3.0,
            time_between_shots: 7.0,
            sprite: AnimatedSprite::new("red", 3, 19, 22)
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
        world_updates
    }

    fn texture_source_rect(&self) -> Rectangle {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite.sheet_path
    }
}