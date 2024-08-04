use raylib::{ffi::KeyboardKey, math::Vector2, RaylibHandle};

use crate::utils::vector_utils::directions_based_direction_vector;

pub trait KeyboardEventsProvider {
    fn is_base_attack_pressed(&self) -> bool;
    fn is_up_pressed(&self) -> bool;
    fn is_right_pressed(&self) -> bool;
    fn is_down_pressed(&self) -> bool;
    fn is_left_pressed(&self) -> bool;
    fn direction_based_on_pressed_keys(&self) -> Option<Vector2>;
}

impl KeyboardEventsProvider for RaylibHandle {
    fn is_base_attack_pressed(&self) -> bool {
        return self.is_key_down(KeyboardKey::KEY_K);
    }

    fn is_up_pressed(&self) -> bool {
        return self.is_key_down(KeyboardKey::KEY_W) || self.is_key_down(KeyboardKey::KEY_UP);
    }
    
    fn is_right_pressed(&self) -> bool {
        return self.is_key_down(KeyboardKey::KEY_D) || self.is_key_down(KeyboardKey::KEY_RIGHT);
    }
    
    fn is_down_pressed(&self) -> bool {
        return self.is_key_down(KeyboardKey::KEY_S) || self.is_key_down(KeyboardKey::KEY_DOWN);
    }

    fn is_left_pressed(&self) -> bool {
        return self.is_key_down(KeyboardKey::KEY_A) || self.is_key_down(KeyboardKey::KEY_LEFT);
    }
    
    fn direction_based_on_pressed_keys(&self) -> Option<Vector2> {
        let up = self.is_up_pressed();
        let right = self.is_right_pressed();
        let down = self.is_down_pressed();
        let left = self.is_left_pressed();
        return directions_based_direction_vector(up, right, down, left);
    }
}