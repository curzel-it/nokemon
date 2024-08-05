use raylib::math::Vector2;

use crate::{constants::{ANIMATION_NAME_FRONT, ANIMATION_NAME_MOVEMENT, ANIMATION_NAME_STILL, DIRECTION_NAME_E, DIRECTION_NAME_N, DIRECTION_NAME_NE, DIRECTION_NAME_NW, DIRECTION_NAME_S, DIRECTION_NAME_SE, DIRECTION_NAME_SW, DIRECTION_NAME_W}, game_engine::{behaviors::EntityBehavior, game::Game}};

#[derive(Debug)]
pub struct UpdateSprites;

impl UpdateSprites {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for UpdateSprites {
    fn update(&self, entity_id: &u32, game: &mut Game, time_since_last_update: f32) {
        let entity = game.entities.get_mut(entity_id).unwrap();        

        if entity.sprite_invalidated {
            if let Some(movement_animation) = self.movement_sprite(entity.speed, entity.direction) {
                entity.change_animation(movement_animation.as_str());
            } else {
                entity.change_animation(ANIMATION_NAME_FRONT);
            }
        }

        entity.current_sprite.update(time_since_last_update);
    }
}

impl UpdateSprites {
    fn movement_sprite(&self, speed: f32, direction: Vector2) -> Option<String> {        
        if let Some(direction_name) = self.direction_name(direction) {
            if speed == 0.0 {
                return Some(format!("{ANIMATION_NAME_STILL}{direction_name}"));
            } else {
                return Some(format!("{ANIMATION_NAME_MOVEMENT}{direction_name}"));
            }
        }
        return None;
    }

    fn direction_name(&self, direction: Vector2) -> Option<&str> {
        if direction.y < 0.0 && direction.x == 0.0 { return Some(DIRECTION_NAME_N); }
        if direction.y < 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_NE); }
        if direction.y == 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_E); }
        if direction.y > 0.0 && direction.x > 0.0 { return Some(DIRECTION_NAME_SE); }
        if direction.y > 0.0 && direction.x == 0.0 { return Some(DIRECTION_NAME_S); }
        if direction.y > 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_SW); }
        if direction.y == 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_W); }
        if direction.y < 0.0 && direction.x < 0.0 { return Some(DIRECTION_NAME_NW); }
        return None
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::ANIMATION_NAME_FRONT, game_engine::{game::Game, game_engine::GameEngine, keyboard_events_provider::NoKeyboard}};
    
    fn test_setup(direction: Vector2) -> (GameEngine, Game, u32) {
        let engine = GameEngine::new();        
        let mut game = Game::test();

        let id = game.add_entity_by_species("red");
        let entity = game.entities.get_mut(&id).unwrap();        
        entity.change_direction(direction);

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
        game.entities.get_mut(&id).unwrap().speed = 0.0;
        assert_eq!(game.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut game, 1.0, &nokb);
        assert_eq!(game.animation_name_of_entity(&id), "stillw"); 
    }
}
