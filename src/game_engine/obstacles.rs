use common_macros::hash_map;
use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ANIMATION_NAME_FRONT, ASSETS_PATH, INFINITE_LIFESPAN}, impl_embodied_entity, sprites::sprite_set::SpriteSet};

use super::{entity::Entity, entity_body::EntityBody, entity_factory::EntityFactory, world::World, world_state_update::WorldStateUpdate};

#[derive(Debug)]
pub struct StaticObstacle {
    body: EntityBody,
    sprite_sheet_path: String,
}

impl StaticObstacle {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body,
            sprite_sheet_path: format!("{}/obstacle.png", ASSETS_PATH)
        }
    }
}

impl_embodied_entity!(StaticObstacle);

impl Entity for StaticObstacle {
    fn update(&mut self, _: &World, _: f32) -> Vec<WorldStateUpdate> {
        vec![]
    }

    fn texture_source_rect(&self) -> Rectangle {
        Rectangle::new(
            0.0,
            0.0,
            self.body.frame.width,
            self.body.frame.height
        )
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite_sheet_path 
    }
}


impl EntityFactory {
    pub fn build_static_obstacle(&self, sprite: String, frame: Rectangle) -> StaticObstacle {
        let sprites = SpriteSet::new(hash_map! {
            ANIMATION_NAME_FRONT.to_string() => vec![sprite.clone()],
        });
        let mut body = self.build_with_sprites(&sprites);
        body.set_animation(ANIMATION_NAME_FRONT);
        body.is_rigid = true;
        body.base_speed = 0.0;
        body.current_speed = 0.0;
        body.lifespan = INFINITE_LIFESPAN;
        body.dp = 0.0;
        body.direction = Vector2::zero();
        body.frame = frame;
        StaticObstacle::new(body)
    }
}