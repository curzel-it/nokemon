use crate::{constants::{KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS, KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS_FIRST}, utils::directions::Direction};

pub const NO_KEYBOARD_EVENTS: KeyboardEventsProvider = KeyboardEventsProvider::new();

pub struct KeyboardEventsProvider {
    pub has_back_been_pressed: bool,
    pub has_menu_been_pressed: bool,
    pub has_confirmation_been_pressed: bool,
    pub has_attack_key_been_pressed: bool,
    pub has_backspace_been_pressed: bool,

    pub direction_up: HoldableKey,
    pub direction_right: HoldableKey,
    pub direction_down: HoldableKey,
    pub direction_left: HoldableKey,

    pub discard_direction_events_until_next_arrow_key_is_pressed: bool,
    pub currently_pressed_character: Option<char>,
}

impl KeyboardEventsProvider {
    pub const fn new() -> Self {
        Self {
            has_back_been_pressed: false,
            has_menu_been_pressed: false,
            has_attack_key_been_pressed: false,
            has_confirmation_been_pressed: false,
            has_backspace_been_pressed: false,
            direction_up: HoldableKey::new(),
            direction_right: HoldableKey::new(),
            direction_down: HoldableKey::new(),
            direction_left: HoldableKey::new(),
            discard_direction_events_until_next_arrow_key_is_pressed: false,
            currently_pressed_character: None,
        }
    }

    pub fn on_world_changed(&mut self) {
        self.discard_direction_events_until_next_arrow_key_is_pressed = true;
    }

    pub fn direction_based_on_current_keys(&self, current: Direction) -> Direction {
        if self.discard_direction_events_until_next_arrow_key_is_pressed {
            return Direction::Unknown;
        }

        let direction_from_new_keys = Direction::from_data(
            self.direction_up.is_down,
            self.direction_right.is_down,
            self.direction_down.is_down,
            self.direction_left.is_down,
        );
        match direction_from_new_keys {
            Direction::Unknown => current,
            Direction::Still => Direction::Unknown,
            _ => direction_from_new_keys
        }
    }

    pub fn is_any_arrow_key_down(&self) -> bool {
        self.direction_up.is_down
            || self.direction_right.is_down
            || self.direction_down.is_down
            || self.direction_left.is_down
    }
}

pub struct HoldableKey {
    time_to_next_press_event: f32,
    pub is_down: bool,
    pub is_pressed: bool,
}

impl HoldableKey {
    const fn new() -> Self {
        Self {
            time_to_next_press_event: 0.0,
            is_down: false,
            is_pressed: false,
        }
    }

    pub fn update(&mut self, is_pressed: bool, is_down: bool, time_since_last_update: f32) {
        self.is_down = is_down;
        self.is_pressed = is_pressed;

        if self.is_pressed {
            self.time_to_next_press_event = KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS_FIRST;
        } else if self.is_down {
            self.time_to_next_press_event -= time_since_last_update;

            if self.time_to_next_press_event <= 0.0 {
                self.is_pressed = true;
                self.time_to_next_press_event = KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS;
            }
        }
    }
}
