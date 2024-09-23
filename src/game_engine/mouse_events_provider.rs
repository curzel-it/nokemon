use raylib::{ffi::MouseButton, RaylibHandle};

use crate::constants::TILE_SIZE;

pub struct MouseEventsProvider {
    pub is_left_down: bool,
    pub has_left_been_pressed: bool,
    pub has_right_been_pressed: bool,
    pub x: i32,
    pub y: i32,
    pub has_moved: bool,
}

impl MouseEventsProvider {
    pub const fn new() -> Self {
        Self {            
            is_left_down: false,
            has_left_been_pressed: false,
            has_right_been_pressed: false,
            x: 0, 
            y: 0,
            has_moved: false
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, rendering_scale: f32) {
        self.is_left_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        self.has_left_been_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        self.has_right_been_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);

        let position = rl.get_mouse_position();
        let new_x = (position.x / (rendering_scale * TILE_SIZE)) as i32;
        let new_y = (position.y / (rendering_scale * TILE_SIZE)) as i32;
        
        self.has_moved = new_x != self.x || new_y != self.y;
        self.x = new_x;
        self.y = new_y;
    }

    pub fn on_world_changed(&mut self) {
        self.has_left_been_pressed = false;
        self.has_right_been_pressed = false;
        self.has_moved = false;
    } 
}