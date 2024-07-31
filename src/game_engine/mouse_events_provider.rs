use raylib::{ffi::MouseButton, math::Vector2, RaylibHandle};

pub trait MouseEventsProvider {
    fn mouse_position(&self) -> Vector2;
    fn is_left_mouse_pressed(&self) -> bool;
    fn is_left_mouse_released(&self) -> bool;
}

impl MouseEventsProvider for RaylibHandle {
    fn mouse_position(&self) -> Vector2 {
        return self.get_mouse_position();
    }

    fn is_left_mouse_pressed(&self) -> bool {
        return self.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    }

    fn is_left_mouse_released(&self) -> bool {
        return self.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
    }
}