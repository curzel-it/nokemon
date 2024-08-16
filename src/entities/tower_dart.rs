
use raylib::math::Rectangle;

use crate::{features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, check_bullet_collisions::handle_collisions_for_bullet, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity, utils::geometry_utils::{Direction, Insets, Scalable}};

#[derive(Debug)]
pub struct TowerDart {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl TowerDart {
    pub fn new(parent: &dyn Entity) -> Self {
        let mut entity = Self {
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: parent.id(),
                frame: Rectangle::new(0.0, 0.0, 10.0, 10.0).to_scale(),
                collision_insets: Insets::zero().to_scale(),
                direction: parent.body().direction,
                current_speed: 5.0,
                base_speed: 5.0,
                hp: 100.0,
                dp: 60.0,
                time_to_next_shot: 5.0,
                time_between_shots: 3.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: false,
                z_index: 0,
                is_ally: parent.body().is_ally,
                is_bullet: true,
                lifespan: 10.0,
            },
            sprite: AnimatedSprite::new("towerdart", 3, 10, 10)
        };
        entity.center_in(&parent.body().frame);
        entity
    }
}

impl_embodied_entity!(TowerDart);

impl Entity for TowerDart {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut handle_collisions_for_bullet(self, world));
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

impl TowerDart {
    fn update_sprite(&mut self, time_since_last_update: f32) {
        let direction = Direction::from_vector(self.body.direction);

        self.sprite.row = match direction {
            Direction::Up => 2.0,
            Direction::Right => 0.0,
            Direction::Down => 3.0,
            Direction::Left => 1.0,
            Direction::Unknown => 3.0,
        };
        self.sprite.update(time_since_last_update);
    }
}