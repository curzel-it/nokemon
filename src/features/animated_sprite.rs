use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ANIMATIONS_FPS, ANIMATION_NAME_MOVEMENT, ANIMATION_NAME_STILL, ASSETS_PATH, DIRECTION_NAME_E, DIRECTION_NAME_N, DIRECTION_NAME_S, DIRECTION_NAME_W}, game_engine::entity::Entity, sprites::timed_content_provider::TimedContentProvider};

pub trait AnimatedEntity: Entity {
    fn sprite_was_invalidated(&self) -> bool;
    fn texture_source_rect(&self) -> Rectangle;
    // fn update_sprite(&mut self, time_since_last_update: f32);
}
/*
#[macro_export]
macro_rules! impl_animated_entity {
    ($struct_name:ident) => {
        impl $crate::features::animated_sprite::AnimatedEntity for $struct_name {
            fn sprite_was_invalidated(&self) -> bool {
                return self.body.sprite_invalidated;
            }
        
            fn update_sprite(&mut self, time_since_last_update: f32) {
                self.body.current_sprite.update(time_since_last_update);
            }
        }
    }
} */

#[derive(Debug)]
pub struct AnimatedSprite {
    pub sheet_path: String,
    pub row: f32,
    pub frames_provider: TimedContentProvider<f32>,
    pub width: f32,
    pub height: f32
}

impl AnimatedSprite {
    pub fn new(sprite: &str, number_of_frames: u32, width: u32, height: u32) -> Self {
        let frames = Vec::from_iter((0..number_of_frames).map(|v| v as f32));

        Self {
            sheet_path: format!("{}/{}.png", ASSETS_PATH, sprite),
            row: 0.0,
            frames_provider: TimedContentProvider::new(frames, ANIMATIONS_FPS),
            width: width as f32,
            height: height as f32
        }
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.frames_provider.update(time_since_last_update)
    }

    pub fn texture_source_rect(&self) -> Rectangle {
        Rectangle::new(
            self.frames_provider.current_frame() * self.width,
            self.row * self.height,
            self.width,
            self.height
        )
    }
}

pub fn update_sprite(entity: &mut dyn Entity, time_since_last_update: f32) {
    /*if entity.sprite_was_invalidated() {
        if let Some(movement_animation) = movement_sprite(entity.body().current_speed, entity.body().direction) {
            entity.body_mut().set_animation(movement_animation.as_str());
        } else {
            entity.body_mut().set_animation(ANIMATION_NAME_FRONT);
        }
    }

    // entity.update_sprite(time_since_last_update);*/
}

fn movement_sprite(speed: f32, direction: Vector2) -> Option<String> {        
    if let Some(direction_name) = direction_name(direction) {
        if speed == 0.0 {
            return Some(format!("{ANIMATION_NAME_STILL}{direction_name}"));
        } else {
            return Some(format!("{ANIMATION_NAME_MOVEMENT}{direction_name}"));
        }
    }
    None
}

fn direction_name(direction: Vector2) -> Option<String> {
    if direction.y < 0.0 && direction.x == 0.0 { return Some(DIRECTION_NAME_N.to_owned()); }
    // if direction.y < 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_NE.to_owned()); }
    if direction.y == 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_E.to_owned()); }
    // if direction.y > 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_SE.to_owned()); }
    if direction.y > 0.0 && direction.x == 0.0 { return Some(DIRECTION_NAME_S.to_owned()); }
    // if direction.y > 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_SW.to_owned()); }
    if direction.y == 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_W.to_owned()); }
    // if direction.y < 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_NW.to_owned()); }
    None
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::{ANIMATION_NAME_FRONT, RECT_ORIGIN_FULL_HD}, game_engine::{world::World, game_engine::GameEngine, simple_entity::SimpleEntity}};
    
    fn test_setup(direction: Vector2) -> (GameEngine, World, u32) {
        let engine = GameEngine::new();        
        let mut world = World::test();
        world.bounds = RECT_ORIGIN_FULL_HD;
        world.camera_viewport = RECT_ORIGIN_FULL_HD;

        let mut body = world.entity_factory.build("red");
        body.direction = direction;
        body.frame.x = 50.0;
        body.frame.y = 50.0;
        body.set_animation(ANIMATION_NAME_FRONT);

        let hero = Box::new(SimpleEntity::new(body));
        let hero_id = world.add_entity(hero);

        (engine, world, hero_id)
    }

    #[test]
    fn can_switch_sprite_when_moving_east() {
        let (engine, mut world, id) = test_setup(Vector2::new(1.0, 0.0));
        assert_eq!(world.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut world, 1.0);
        assert_eq!(world.animation_name_of_entity(&id), "walke");        
    }

    #[test]
    fn can_switch_sprite_when_moving_west() {
        let (engine, mut world, id) = test_setup(Vector2::new(-1.0, 0.0));
        assert_eq!(world.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut world, 1.0);
        assert_eq!(world.animation_name_of_entity(&id), "walkw");        
    }

    #[test]
    fn can_show_directional_still_sprite_when_speed_is_zero() {
        let (engine, mut world, id) = test_setup(Vector2::new(-1.0, 0.0));
        
        let mut entities = world.entities.borrow_mut();
        let entity = entities.get_mut(&id).unwrap();  
        entity.body_mut().current_speed = 0.0;
        drop(entities);     

        assert_eq!(world.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut world, 1.0);
        assert_eq!(world.animation_name_of_entity(&id), "stillw"); 
    }
}