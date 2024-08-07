use crate::game_engine::{entity::Entity, keyboard_events_provider::KeyboardState};

pub fn set_direction_according_to_keyboard_state(entity: &mut dyn Entity, keyboard_state: &KeyboardState) {
    let new_direction = keyboard_state.direction_based_on_pressed_keys;

    if let Some(new_direction) = new_direction {
        entity.reset_speed();
        entity.set_direction(new_direction);
    } else {
        entity.set_speed(0.0);
    }
}