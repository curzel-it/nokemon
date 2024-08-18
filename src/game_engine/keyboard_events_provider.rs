use raylib::{ffi::KeyboardKey, math::Vector2, RaylibHandle};

use crate::utils::vector_utils::directions_based_direction_vector_4d;

#[derive(Default, Clone, Copy)]
pub struct KeyboardState {
    pub is_up_down: bool,
    pub is_right_down: bool,
    pub is_down_down: bool, 
    pub is_left_down: bool,
    pub has_inventory_been_pressed: bool,
    pub has_confirmation_been_pressed: bool,
    pub has_up_been_pressed: bool,
    pub has_right_been_pressed: bool,
    pub has_down_been_pressed: bool,
    pub has_left_been_pressed: bool,
}

impl KeyboardState {
    pub fn nothing() -> Self {
        Self {
            is_up_down: false,
            is_right_down: false,
            is_down_down: false, 
            is_left_down: false,
            has_inventory_been_pressed: false,
            has_confirmation_been_pressed: false,
            has_up_been_pressed: false,
            has_right_been_pressed: false,
            has_down_been_pressed: false,
            has_left_been_pressed: false,
        }
    }

    pub fn direction_based_on_down_keys(&self) -> Option<Vector2> {
        directions_based_direction_vector_4d(
            self.is_up_down, 
            self.is_right_down, 
            self.is_down_down, 
            self.is_left_down
        )
    }
}

pub trait KeyboardEventsProvider {
    fn state(&self) -> KeyboardState;
}

impl KeyboardEventsProvider for RaylibHandle {
    fn state(&self) -> KeyboardState {
        KeyboardState {
            is_up_down: self.is_key_down(KeyboardKey::KEY_W) || self.is_key_down(KeyboardKey::KEY_UP),
            is_right_down: self.is_key_down(KeyboardKey::KEY_D) || self.is_key_down(KeyboardKey::KEY_RIGHT),
            is_down_down: self.is_key_down(KeyboardKey::KEY_S) || self.is_key_down(KeyboardKey::KEY_DOWN),
            is_left_down: self.is_key_down(KeyboardKey::KEY_A) || self.is_key_down(KeyboardKey::KEY_LEFT),
            has_inventory_been_pressed: self.is_key_pressed(KeyboardKey::KEY_I),
            has_confirmation_been_pressed: self.is_key_pressed(KeyboardKey::KEY_SPACE),
            has_up_been_pressed: self.is_key_pressed(KeyboardKey::KEY_W) || self.is_key_pressed(KeyboardKey::KEY_UP),
            has_right_been_pressed: self.is_key_pressed(KeyboardKey::KEY_D) || self.is_key_pressed(KeyboardKey::KEY_RIGHT),
            has_down_been_pressed: self.is_key_pressed(KeyboardKey::KEY_S) || self.is_key_pressed(KeyboardKey::KEY_DOWN),
            has_left_been_pressed: self.is_key_pressed(KeyboardKey::KEY_A) || self.is_key_pressed(KeyboardKey::KEY_LEFT),
        }
    }
}

pub struct NoKeyboardEvents {}

impl KeyboardEventsProvider for NoKeyboardEvents {
    fn state(&self) -> KeyboardState {
        KeyboardState::nothing()
    }
}