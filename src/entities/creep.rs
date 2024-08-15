use raylib::math::{Rectangle, Vector2};

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, linear_movement::move_linearly, position_seeker::set_direction_towards}, game_engine::{entity::Entity, entity_body::EntityBody, entity_factory::get_next_entity_id, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity, sprites::{sprite::Sprite, sprite_set::SpriteSet}, utils::geometry_utils::{Direction, Insets, Scalable}};

#[derive(Debug)]
pub struct Creep {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl Creep {
    pub fn new() -> Self {
        Self {             
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame: Rectangle::new(0.0, 0.0, 19.0, 22.0).to_scale(),
                collision_insets: Insets::new(12.0, 4.0, 0.0, 4.0),
                direction: Vector2::zero(),
                current_speed: 1.5,
                base_speed: 1.5,
                hp: 100.0,
                dp: 0.0,
                sprite_set: SpriteSet::default(),
                current_sprite: Sprite::empty(),
                sprite_invalidated: true,
                time_to_next_shot: 5.0,
                time_between_shots: 3.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: true,
                z_index: 0,
                is_ally: false,
                is_bullet: false,
                lifespan: INFINITE_LIFESPAN,
            },
            sprite: AnimatedSprite::new("white", 3, 19, 22)
        }
    }
}

impl_embodied_entity!(Creep);

impl Entity for Creep {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        set_direction_towards(self, &world.cached_hero_position);
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
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

impl Creep {
    fn update_sprite(&mut self, time_since_last_update: f32) {
        if self.body.sprite_invalidated {
            let direction = Direction::from_vector(self.body.direction);
            let is_moving = self.body.current_speed != 0.0;

            self.sprite.row = match (direction, is_moving) {
                (Direction::Up, true) => 0.0,
                (Direction::Up, false) => 1.0,
                (Direction::Right, true) => 2.0,
                (Direction::Right, false) => 3.0,
                (Direction::Down, true) => 4.0,
                (Direction::Down, false) => 5.0,
                (Direction::Left, true) => 6.0,
                (Direction::Left, false) => 7.0,
                (Direction::Unknown, true) => 5.0,
                (Direction::Unknown, false) => 5.0
            };
        }    
        self.sprite.update(time_since_last_update);
    }
}