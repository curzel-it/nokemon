use raylib::math::Vector2;

use crate::{constants::{ANIMATION_NAME_FRONT, ANIMATION_NAME_MOVEMENT, ANIMATION_NAME_STILL, DIRECTION_NAME_E, DIRECTION_NAME_N, DIRECTION_NAME_NE, DIRECTION_NAME_NW, DIRECTION_NAME_S, DIRECTION_NAME_SE, DIRECTION_NAME_SW, DIRECTION_NAME_W}, game_engine::{entity::Entity, game::Game}};

pub trait AnimatedEntity: Entity {
    fn sprite_was_invalidated(&self) -> bool;
    fn update_sprite(&mut self, time_since_last_update: f32);
}

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
}

pub fn update_sprite(entity: &mut dyn AnimatedEntity, time_since_last_update: f32) {
    if entity.sprite_was_invalidated() {
        if let Some(movement_animation) = movement_sprite(entity.speed(), entity.direction()) {
            entity.set_animation(movement_animation.as_str());
        } else {
            entity.set_animation(ANIMATION_NAME_FRONT);
        }
    }

    entity.update_sprite(time_since_last_update);
}

fn movement_sprite(speed: f32, direction: Vector2) -> Option<String> {        
    if let Some(direction_name) = direction_name(direction) {
        if speed == 0.0 {
            return Some(format!("{ANIMATION_NAME_STILL}{direction_name}"));
        } else {
            return Some(format!("{ANIMATION_NAME_MOVEMENT}{direction_name}"));
        }
    }
    return None;
}

fn direction_name(direction: Vector2) -> Option<String> {
    if direction.y < 0.0 && direction.x == 0.0 { return Some(DIRECTION_NAME_N.to_owned()); }
    if direction.y < 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_NE.to_owned()); }
    if direction.y == 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_E.to_owned()); }
    if direction.y > 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_SE.to_owned()); }
    if direction.y > 0.0 && direction.x == 0.0 { return Some(DIRECTION_NAME_S.to_owned()); }
    if direction.y > 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_SW.to_owned()); }
    if direction.y == 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_W.to_owned()); }
    if direction.y < 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_NW.to_owned()); }
    return None
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::ANIMATION_NAME_FRONT, game_engine::{game::Game, game_engine::GameEngine, keyboard_events_provider::NoKeyboard}};
    
    fn test_setup(direction: Vector2) -> (GameEngine, Game, u32) {
        let engine = GameEngine::new();        
        let mut game = Game::test();

        let id = game.add_entity_by_species("red");
        let mut entities = game.entities.borrow_mut();
        let entity = entities.get_mut(&id).unwrap();        
        entity.set_direction(direction);

        drop(entities);
        return (engine, game, id);
    }

    #[test]
    fn can_switch_sprite_when_moving_east() {
        let nokb = NoKeyboard {};
        let (engine, mut game, id) = test_setup(Vector2::new(1.0, 0.0));
        assert_eq!(game.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut game, 1.0, &nokb);
        assert_eq!(game.animation_name_of_entity(&id), "walke");        
    }

    #[test]
    fn can_switch_sprite_when_moving_west() {
        let nokb = NoKeyboard {};
        let (engine, mut game, id) = test_setup(Vector2::new(-1.0, 0.0));
        assert_eq!(game.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut game, 1.0, &nokb);
        assert_eq!(game.animation_name_of_entity(&id), "walkw");        
    }

    #[test]
    fn can_show_directional_still_sprite_when_speed_is_zero() {
        let nokb = NoKeyboard {};
        let (engine, mut game, id) = test_setup(Vector2::new(-1.0, 0.0));
        
        let mut entities = game.entities.borrow_mut();
        let entity = entities.get_mut(&id).unwrap();  
        entity.set_speed(0.0);
        drop(entities);     

        assert_eq!(game.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut game, 1.0, &nokb);
        assert_eq!(game.animation_name_of_entity(&id), "stillw"); 
    }
}