use raylib::{ffi::KeyboardKey, math::Vector2, RaylibHandle};

use crate::utils::vector_utils::directions_based_direction_vector_4d;

#[derive(Default, Clone, Copy)]
pub struct KeyboardState {
    pub direction_based_on_pressed_keys: Option<Vector2>
}

pub trait KeyboardEventsProvider {
    fn keyboard_state(&self) -> KeyboardState;
}

impl KeyboardEventsProvider for RaylibHandle {
    fn keyboard_state(&self) -> KeyboardState {
        let is_up_pressed = self.is_key_down(KeyboardKey::KEY_W) || self.is_key_down(KeyboardKey::KEY_UP);
        let is_right_pressed = self.is_key_down(KeyboardKey::KEY_D) || self.is_key_down(KeyboardKey::KEY_RIGHT);
        let is_down_pressed = self.is_key_down(KeyboardKey::KEY_S) || self.is_key_down(KeyboardKey::KEY_DOWN);
        let is_left_pressed = self.is_key_down(KeyboardKey::KEY_A) || self.is_key_down(KeyboardKey::KEY_LEFT);

        let direction = directions_based_direction_vector_4d(
            is_up_pressed, 
            is_right_pressed, 
            is_down_pressed, 
            is_left_pressed
        );

        KeyboardState {
            direction_based_on_pressed_keys: direction
        }
    }
}

pub struct NoKeyboard;

impl KeyboardEventsProvider for NoKeyboard {
    fn keyboard_state(&self) -> KeyboardState {
        KeyboardState::default()
    }
}