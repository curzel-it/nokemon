use raylib::{ffi::KeyboardKey, math::Vector2, RaylibHandle};

use crate::utils::vector_utils::directions_based_direction_vector_4d;

#[derive(Default, Clone, Copy)]
pub struct KeyboardState {
    pub is_up_pressed: bool,
    pub is_right_pressed: bool,
    pub is_down_pressed: bool, 
    pub is_left_pressed: bool,
    pub is_i_pressed: bool,
}

impl KeyboardState {
    pub fn nothing() -> Self {
        Self {
            is_up_pressed: false,
            is_right_pressed: false,
            is_down_pressed: false, 
            is_left_pressed: false,
            is_i_pressed: false,
        }
    }

    pub fn should_toggle_inventory(&self) -> bool {
        self.is_i_pressed
    }

    pub fn direction_based_on_pressed_keys(&self) -> Option<Vector2> {
        directions_based_direction_vector_4d(
            self.is_up_pressed, 
            self.is_right_pressed, 
            self.is_down_pressed, 
            self.is_left_pressed
        )
    }
}

pub trait KeyboardEventsProvider {
    fn state(&self) -> KeyboardState;
}

impl KeyboardEventsProvider for RaylibHandle {
    fn state(&self) -> KeyboardState {
        KeyboardState {
            is_up_pressed: self.is_key_down(KeyboardKey::KEY_W) || self.is_key_down(KeyboardKey::KEY_UP),
            is_right_pressed: self.is_key_down(KeyboardKey::KEY_D) || self.is_key_down(KeyboardKey::KEY_RIGHT),
            is_down_pressed: self.is_key_down(KeyboardKey::KEY_S) || self.is_key_down(KeyboardKey::KEY_DOWN),
            is_left_pressed: self.is_key_down(KeyboardKey::KEY_A) || self.is_key_down(KeyboardKey::KEY_LEFT),
            is_i_pressed: self.is_key_pressed(KeyboardKey::KEY_I),
        }
    }
}

pub struct NoKeyboardEvents {}

impl KeyboardEventsProvider for NoKeyboardEvents {
    fn state(&self) -> KeyboardState {
        KeyboardState::nothing()
    }
}