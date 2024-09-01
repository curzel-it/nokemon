use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::{constants::KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS, utils::directions::Direction};

pub const NO_KEYBOARD_EVENTS: KeyboardEventsProvider = KeyboardEventsProvider::new();

pub struct KeyboardEventsProvider {
    pub has_back_been_pressed: bool,
    pub has_menu_been_pressed: bool,
    pub has_confirmation_been_pressed: bool,

    pub direction_up: HoldableKey,
    pub direction_right: HoldableKey,
    pub direction_down: HoldableKey,
    pub direction_left: HoldableKey,
}

impl KeyboardEventsProvider {
    pub const fn new() -> Self {
        Self {            
            has_back_been_pressed: false,
            has_menu_been_pressed: false,
            has_confirmation_been_pressed: false,
            direction_up: HoldableKey::new(KeyboardKey::KEY_W, KeyboardKey::KEY_UP),
            direction_right: HoldableKey::new(KeyboardKey::KEY_D, KeyboardKey::KEY_RIGHT),
            direction_down: HoldableKey::new(KeyboardKey::KEY_S, KeyboardKey::KEY_DOWN),
            direction_left: HoldableKey::new(KeyboardKey::KEY_A, KeyboardKey::KEY_LEFT),
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, time_since_last_update: f32) {
        self.has_back_been_pressed = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE);
        self.has_menu_been_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
        self.has_confirmation_been_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);        
        self.direction_up.update(rl, time_since_last_update);
        self.direction_right.update(rl, time_since_last_update);
        self.direction_down.update(rl, time_since_last_update);
        self.direction_left.update(rl, time_since_last_update);        
    }
}

pub struct HoldableKey {
    key1: KeyboardKey,
    key2: KeyboardKey,
    time_to_next_press_event: f32,
    pub is_down: bool,
    pub is_pressed: bool
}

impl HoldableKey {
    const fn new(key1: KeyboardKey, key2: KeyboardKey) -> Self {
        Self {
            key1, key2, 
            time_to_next_press_event: 0.0, 
            is_down: false, 
            is_pressed: false
        }
    }

    fn update(&mut self, rl: &RaylibHandle, time_since_last_update: f32) {
        self.is_down = rl.is_key_down(self.key1) || rl.is_key_down(self.key2);
        self.is_pressed = rl.is_key_pressed(self.key1) || rl.is_key_pressed(self.key2);
        
        if self.is_pressed {
            self.time_to_next_press_event = KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS;
        } else if self.is_down {
            self.time_to_next_press_event -= time_since_last_update;                

            if self.time_to_next_press_event <= 0.0 {
                self.is_pressed = true;
                self.time_to_next_press_event = KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS;
            }
        }
    }
}

impl KeyboardEventsProvider {
    pub fn direction_based_on_current_keys(&self, current: Direction) -> Direction {
        let direction_from_new_keys = Direction::from_data(
            !matches!(current, Direction::Up) && self.direction_up.is_down, 
            !matches!(current, Direction::Right) && self.direction_right.is_down, 
            !matches!(current, Direction::Down) && self.direction_down.is_down, 
            !matches!(current, Direction::Left) && self.direction_left.is_down
        );
        if direction_from_new_keys != current && direction_from_new_keys != Direction::Unknown {
            return direction_from_new_keys;
        }
        Direction::from_data(
            self.direction_up.is_down, 
            self.direction_right.is_down, 
            self.direction_down.is_down, 
            self.direction_left.is_down
        )
    }

    pub fn is_any_arrow_key_down(&self) -> bool {
        self.direction_up.is_down || self.direction_right.is_down || self.direction_down.is_down || self.direction_left.is_down
    }
}
