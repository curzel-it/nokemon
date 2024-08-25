use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::utils::{vector::Vector2d, vector_utils::directions_based_direction_vector_4d};

#[derive(Default, Clone, Copy)]
pub struct KeyboardState {
    pub is_up_down: bool,
    pub is_right_down: bool,
    pub is_down_down: bool, 
    pub is_left_down: bool,
    pub has_back_been_pressed: bool,
    pub has_menu_been_pressed: bool,
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
            has_back_been_pressed: false,
            has_menu_been_pressed: false,
            has_confirmation_been_pressed: false,
            has_up_been_pressed: false,
            has_right_been_pressed: false,
            has_down_been_pressed: false,
            has_left_been_pressed: false,
        }
    }

    pub fn direction_based_on_down_keys(&self, current: &Vector2d) -> Option<Vector2d> {
        if !self.is_any_arrow_key_down() {
            return None
        }

        let direction = directions_based_direction_vector_4d(
            current.y >= 0.0 && self.is_up_down, 
            current.x <= 0.0 && self.is_right_down, 
            current.y <= 0.0 && self.is_down_down, 
            current.x >= 0.0 && self.is_left_down
        );
        Some(direction.unwrap_or(*current))
    }

    fn is_any_arrow_key_down(&self) -> bool {
        self.is_up_down || self.is_right_down || self.is_down_down || self.is_left_down
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
            has_back_been_pressed: self.is_key_pressed(KeyboardKey::KEY_ESCAPE),
            has_menu_been_pressed: self.is_key_pressed(KeyboardKey::KEY_ENTER),
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