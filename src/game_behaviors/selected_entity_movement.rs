use crate::game_engine::{behaviors::GameBehavior, game::Game};

pub struct SelectedEntityMovement;

impl GameBehavior for SelectedEntityMovement {
    fn update(&self, game: &mut Game, _: f32) {        
        let new_direction = game.keyboard_state.direction_based_on_pressed_keys;

        if let Some(entity) = game.selected_entity_mut() {
            if let Some(new_direction) = new_direction {
                entity.reset_speed();
                entity.change_direction(new_direction);
            } else {
                entity.speed = 0.0;
            }
        }
    }
}

impl SelectedEntityMovement {
    pub fn new() -> Self {
        Self {}
    }
}