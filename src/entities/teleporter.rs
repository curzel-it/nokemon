use core::time;

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{HERO_ENTITY_ID, INFINITE_LIFESPAN, NO_PARENT, TILE_SIZE}, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, levels::LEVEL_ID_HOUSE_INTERIOR, linear_movement::move_linearly}, game_engine::{collision_detection::Collision, entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, impl_single_animation_sprite_update, utils::geometry_utils::{is_collision_trajectory, Insets}};

#[derive(Debug)]
pub struct Teleporter {
    body: EntityBody,
    destination: u32,
    sprite: AnimatedSprite,
}

impl Teleporter {
    pub fn new() -> Self {
        Self { 
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame: Rectangle::new(0.0, 0.0, TILE_SIZE, TILE_SIZE),
                collision_insets: Insets::zero(),
                direction: Vector2::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: false,
                z_index: 0,
                is_ally: false,
                is_bullet: false,
                lifespan: INFINITE_LIFESPAN,
            },
            destination: LEVEL_ID_HOUSE_INTERIOR,
            sprite: AnimatedSprite::new("white", 3, TILE_SIZE as u32, TILE_SIZE as u32),
        }
    }
}

impl_embodied_entity!(Teleporter);
impl_single_animation_sprite_update!(Teleporter);

impl Entity for Teleporter {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        self.update_sprite(time_since_last_update);

        if self.should_teleport(world) {
            return vec![self.engine_update_push_world()];
        }
        vec![]
    }

    fn texture_source_rect(&self) -> Rectangle {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite.sheet_path 
    }
}

impl Teleporter {
    fn should_teleport(&self, world: &World) -> bool {
        if self.hero_collision(world).is_none() {
            return false 
        }
        let is_colliding = is_collision_trajectory(
            &world.cached_hero_props.direction, 
            &world.cached_hero_props.frame, 
            &self.body.frame
        );
        is_colliding && world.cached_hero_props.speed > 0.1
    }

    fn hero_collision(&self, world: &World) -> Option<Collision> {
        let no_collisions = vec![];

        let collision = world.collisions
            .get(&self.body.id)
            .unwrap_or(&no_collisions)
            .into_iter()
            .find(|c| c.other_id == HERO_ENTITY_ID);

        return if let Some(collision) = collision {
            Some(collision.clone())
        } else {
            None
        }            
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::ToggleWorld(self.destination)
        )
    }
}