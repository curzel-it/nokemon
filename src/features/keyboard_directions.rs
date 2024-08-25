use crate::{constants::STEP_INPUT_THRESHOLD, game_engine::{entity::Entity, keyboard_events_provider::KeyboardState}};

pub fn set_direction_according_to_keyboard_state(entity: &mut dyn Entity, keyboard_state: &KeyboardState) {
    let offset = entity.body().offset;
    let current_direction = entity.body().direction;
    let new_direction = keyboard_state.direction_based_on_down_keys(&current_direction);

    if offset.x.abs() < STEP_INPUT_THRESHOLD && offset.y.abs() < STEP_INPUT_THRESHOLD {
        if let Some(new_direction) = new_direction {
            entity.body_mut().reset_speed();
            entity.body_mut().direction = new_direction;
        } else {
            entity.body_mut().current_speed = 0.0;
        }
    } else {
        entity.body_mut().direction = current_direction;
    }
}