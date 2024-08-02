use raylib::math::Vector2;

use crate::{constants::{ANIMATION_NAME_FRONT, ANIMATION_NAME_MOVEMENT_E, ANIMATION_NAME_MOVEMENT_N, ANIMATION_NAME_MOVEMENT_NE, ANIMATION_NAME_MOVEMENT_NW, ANIMATION_NAME_MOVEMENT_S, ANIMATION_NAME_MOVEMENT_SE, ANIMATION_NAME_MOVEMENT_SW, ANIMATION_NAME_MOVEMENT_W}, game_engine::{game::Game, behaviors::EntityBehavior}};

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
            if let Some(movement_animation) = self.movement_sprite(entity.direction) {
                entity.change_animation(movement_animation);
            } else {
                entity.change_animation(ANIMATION_NAME_FRONT);
            }
        }

        entity.current_sprite.update(time_since_last_update);
    }
}

impl UpdateSprites {
    fn movement_sprite(&self, direction: Vector2) -> Option<&str> {
        if direction.y < 0.0 && direction.x == 0.0 { return Some(ANIMATION_NAME_MOVEMENT_N); }
        if direction.y < 0.0 && direction.x > 0.0 { return Some(ANIMATION_NAME_MOVEMENT_NE); }
        if direction.y == 0.0 && direction.x > 0.0 { return Some(ANIMATION_NAME_MOVEMENT_E); }
        if direction.y > 0.0 && direction.x > 0.0 { return Some(ANIMATION_NAME_MOVEMENT_SE); }
        if direction.y > 0.0 && direction.x == 0.0 { return Some(ANIMATION_NAME_MOVEMENT_S); }
        if direction.y > 0.0 && direction.x < 0.0 { return Some(ANIMATION_NAME_MOVEMENT_SW); }
        if direction.y == 0.0 && direction.x < 0.0 { return Some(ANIMATION_NAME_MOVEMENT_W); }
        if direction.y < 0.0 && direction.x < 0.0 { return Some(ANIMATION_NAME_MOVEMENT_NW); }
        return None
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::{ANIMATION_NAME_FRONT, ANIMATION_NAME_MOVEMENT_E, ANIMATION_NAME_MOVEMENT_W}, game_engine::{game::Game, game_engine::GameEngine}};
    
    fn test_setup(direction: Vector2) -> (GameEngine, Game, u32) {
        let engine = GameEngine::new();        
        let mut game = Game::test();

        let id = game.add_entity_by_species("red");
        let entity = game.entities.get_mut(&id).unwrap();        
        entity.change_direction(direction);

        return (engine, game, id);
    }

    #[test]
    fn test_can_switch_sprite_when_moving_east() {
        let (engine, mut game, id) = test_setup(Vector2::new(1.0, 0.0));
        assert_eq!(game.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut game, 1.0);
        assert_eq!(game.animation_name_of_entity(&id), ANIMATION_NAME_MOVEMENT_E);        
    }

    #[test]
    fn test_can_switch_sprite_when_moving_west() {
        let (engine, mut game, id) = test_setup(Vector2::new(-1.0, 0.0));
        assert_eq!(game.animation_name_of_entity(&id), ANIMATION_NAME_FRONT);
        engine.update(&mut game, 1.0);
        assert_eq!(game.animation_name_of_entity(&id), ANIMATION_NAME_MOVEMENT_W);        
    }
}
