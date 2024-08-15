use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ANIMATIONS_FPS, ASSETS_PATH, HERO_ENTITY_ID, INFINITE_LIFESPAN, NO_PARENT}, features::{animated_sprite::{update_sprite, AnimatedEntity}, autoremove::remove_automatically, keyboard_directions::set_direction_according_to_keyboard_state, linear_movement::move_linearly, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::{get_next_entity_id, EntityFactory}, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity, sprites::{sprite::Sprite, sprite_set::SpriteSet, timed_content_provider::TimedContentProvider}, utils::geometry_utils::{Direction, Insets}};

use super::surrounding_area_attack::SurroundingAreaAttack;

#[derive(Debug)]
pub struct Hero {
    body: EntityBody,
    time_to_next_shot: f32,
    sprite_sheet_path: String,
    sprite_index: f32,
    sprite_frame_provider: TimedContentProvider<f32>
}

impl Hero {
    pub fn new() -> Self {
        Self { 
            body: EntityBody {
                id: HERO_ENTITY_ID,
                parent_id: NO_PARENT,
                frame: Rectangle::new(0.0, 0.0, 19.0, 22.0),
                collision_insets: Insets::new(12.0, 4.0, 0.0, 4.0),
                direction: Vector2::new(0.0, 0.0),
                current_speed: 3.0,
                base_speed: 3.0,
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
                is_ally: true,
                is_bullet: false,
                lifespan: INFINITE_LIFESPAN,
            },
            time_to_next_shot: 5.0,
            sprite_sheet_path: format!("{}/red.png", ASSETS_PATH),
            sprite_index: 0.0,
            sprite_frame_provider: TimedContentProvider::new(vec![0.0, 1.0, 2.0], ANIMATIONS_FPS)
        }
    }
}

impl_embodied_entity!(Hero);

impl Shooter for Hero {
    fn time_to_next_shot(&self) -> f32 {
        self.time_to_next_shot
    }
    
    fn inc_time_to_next_shot(&mut self, delta: f32) {
        self.time_to_next_shot += delta;
    }
    
    fn reset_time_to_next_shot(&mut self) {
        self.time_to_next_shot = self.body().time_between_shots;
    }
    
    fn create_bullet(&self, entity_factory: &EntityFactory) -> Box<dyn Entity> {
        Box::new(SurroundingAreaAttack::new(self, entity_factory))
    }
}

impl Entity for Hero {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        set_direction_according_to_keyboard_state(self, &world.keyboard_state);
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut shoot_stuff(self, world, time_since_last_update));
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }

    fn texture_source_rect(&self) -> Rectangle {
        Rectangle::new(
            self.sprite_frame_provider.current_frame() * self.body.frame.width,
            self.sprite_index * self.body.frame.height,
            self.body.frame.width,
            self.body.frame.height
        )
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite_sheet_path
    }
}

impl Hero {
    fn update_sprite(&mut self, time_since_last_update: f32) {
        if self.body.sprite_invalidated {
            let direction = Direction::from_vector(self.body.direction);
            let is_moving = self.body.current_speed != 0.0;

            self.sprite_index = match (direction, is_moving) {
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
        self.sprite_frame_provider.update(time_since_last_update);
    }
}