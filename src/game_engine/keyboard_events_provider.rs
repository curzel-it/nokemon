use raylib::{ffi::KeyboardKey, math::Vector2, RaylibHandle};

pub trait KeyboardEventsProvider {
    fn is_up_pressed(&self) -> bool;
    fn is_right_pressed(&self) -> bool;
    fn is_down_pressed(&self) -> bool;
    fn is_left_pressed(&self) -> bool;
    fn direction_based_on_pressed_keys(&self) -> Vector2;
}

impl KeyboardEventsProvider for RaylibHandle {
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
    
    fn direction_based_on_pressed_keys(&self) -> Vector2 {
        let up = self.is_up_pressed();
        let right = self.is_right_pressed();
        let down = self.is_down_pressed();
        let left = self.is_left_pressed();

        if up && left { return Vector2::new(-0.71, -0.71); }
        if up && right { return Vector2::new(0.71, -0.71); }
        if down && left { return Vector2::new(-0.71, 0.71); }
        if down && right { return Vector2::new(0.71, 0.71); }
        
        if up { return Vector2::new(0.0, -1.0); }
        if right { return Vector2::new(1.0, 0.0); }
        if down { return Vector2::new(0.0, 1.0); }
        if left { return Vector2::new(-1.0, -0.0); }

        return Vector2::zero();
    }
}