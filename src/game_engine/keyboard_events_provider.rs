use raylib::{ffi::KeyboardKey, RaylibHandle};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::constants::KEY_BINDINGS_PATH;
use crate::{constants::{KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS, KEYBOARD_KEY_HOLD_TIME_TO_NEXT_PRESS_FIRST}, utils::directions::Direction};

pub const NO_KEYBOARD_EVENTS: KeyboardEventsProvider = KeyboardEventsProvider::new();

lazy_static! {
    pub static ref KEY_BINDINGS: Mutex<KeyBindings> = load_key_bindings();
}

fn load_key_bindings() -> Mutex<KeyBindings> {
    let mut bindings = KeyBindings::default();
    bindings.load();
    Mutex::new(bindings)
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum GameAction {
    MoveUp = 0,
    MoveDown = 1,
    MoveLeft = 2,
    MoveRight = 3,
    Confirm = 4,
    Cancel = 5,
    Attack = 6,
    Menu = 7,
    Backspace = 8,
}

impl GameAction {
    fn from_u32(value: u32) -> Option<GameAction> {
        match value {
            0 => Some(GameAction::MoveUp),
            1 => Some(GameAction::MoveDown),
            2 => Some(GameAction::MoveLeft),
            3 => Some(GameAction::MoveRight),
            4 => Some(GameAction::Confirm),
            5 => Some(GameAction::Cancel),
            6 => Some(GameAction::Attack),
            7 => Some(GameAction::Menu),
            8 => Some(GameAction::Backspace),
            _ => None,
        }
    }
}

pub struct KeyBindings {
    bindings: HashMap<GameAction, Vec<KeyboardKey>>,
}

impl KeyBindings {
    pub fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(GameAction::MoveUp, vec![KeyboardKey::KEY_W, KeyboardKey::KEY_UP]);
        bindings.insert(GameAction::MoveDown, vec![KeyboardKey::KEY_S, KeyboardKey::KEY_DOWN]);
        bindings.insert(GameAction::MoveLeft, vec![KeyboardKey::KEY_A, KeyboardKey::KEY_LEFT]);
        bindings.insert(GameAction::MoveRight, vec![KeyboardKey::KEY_D, KeyboardKey::KEY_RIGHT]);
        bindings.insert(GameAction::Confirm, vec![KeyboardKey::KEY_E, KeyboardKey::KEY_ENTER]);
        bindings.insert(GameAction::Cancel, vec![KeyboardKey::KEY_ESCAPE, KeyboardKey::KEY_BACKSPACE]);
        bindings.insert(GameAction::Attack, vec![KeyboardKey::KEY_SPACE]);
        bindings.insert(GameAction::Menu, vec![KeyboardKey::KEY_ENTER]);
        bindings.insert(GameAction::Backspace, vec![KeyboardKey::KEY_BACKSPACE]);

        Self { bindings }
    }

    pub fn get_keys(&self, action: GameAction) -> Option<&Vec<KeyboardKey>> {
        self.bindings.get(&action)
    }

    pub fn set_keys(&mut self, action: GameAction, keys: Vec<KeyboardKey>) {
        self.bindings.insert(action, keys);
        self.save();
    }

    pub fn is_action_pressed(&self, rl: &RaylibHandle, action: GameAction) -> bool {
        if let Some(keys) = self.get_keys(action) {
            keys.iter().any(|&key| rl.is_key_pressed(key))
        } else {
            false
        }
    }

    pub fn is_action_down(&self, rl: &RaylibHandle, action: GameAction) -> bool {
        if let Some(keys) = self.get_keys(action) {
            keys.iter().any(|&key| rl.is_key_down(key))
        } else {
            false
        }
    }

    fn save(&self) {
        let mut serializable_bindings = HashMap::new();

        for (action, keys) in &self.bindings {
            let action_value = *action as u32;
            let keys_values: Vec<i32> = keys.iter().map(|&key| key as i32).collect();
            serializable_bindings.insert(action_value, keys_values);
        }

        let serialized = serde_json::to_string(&serializable_bindings).unwrap();

        let mut file = File::create(KEY_BINDINGS_PATH).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn load(&mut self) {
        let mut file = File::open(KEY_BINDINGS_PATH).unwrap();
        let mut serialized = String::new();
        file.read_to_string(&mut serialized).unwrap();

        let serializable_bindings: HashMap<u32, Vec<i32>> = serde_json::from_str(&serialized).unwrap();

        let mut bindings = HashMap::new();

        for (action_value, keys_values) in serializable_bindings {
            if let Some(action) = GameAction::from_u32(action_value) {
                let keys: Vec<KeyboardKey> = keys_values
                    .into_iter()
                    .filter_map(|k| keyboard_key_from_i32(k))
                    .collect();
                bindings.insert(action, keys);
            }
        }

        self.bindings = bindings;
    }
}

fn keyboard_key_from_i32(value: i32) -> Option<KeyboardKey> {
    if (KeyboardKey::KEY_NULL as i32) <= value && value <= (KeyboardKey::KEY_KP_EQUAL as i32) {
        Some(unsafe { std::mem::transmute(value) })
    } else {
        None
    }
}

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

    discard_direction_events_until_next_arrow_key_is_pressed: bool,
    pub currently_pressed_character: Option<char>,
    pub currently_pressed_key: Option<KeyboardKey>,
}

impl KeyboardEventsProvider {
    pub const fn new() -> Self {
        Self {
            has_back_been_pressed: false,
            has_menu_been_pressed: false,
            has_attack_key_been_pressed: false,
            has_confirmation_been_pressed: false,
            has_backspace_been_pressed: false,
            direction_up: HoldableKey::new(GameAction::MoveUp),
            direction_right: HoldableKey::new(GameAction::MoveRight),
            direction_down: HoldableKey::new(GameAction::MoveDown),
            direction_left: HoldableKey::new(GameAction::MoveLeft),
            discard_direction_events_until_next_arrow_key_is_pressed: false,
            currently_pressed_character: None,
            currently_pressed_key: None,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, time_since_last_update: f32) {
        let key_bindings = KEY_BINDINGS.lock().unwrap();

        self.discard_direction_events_until_next_arrow_key_is_pressed =
            self.discard_direction_events_until_next_arrow_key_is_pressed
                && !key_bindings.is_action_pressed(rl, GameAction::MoveUp)
                && !key_bindings.is_action_pressed(rl, GameAction::MoveRight)
                && !key_bindings.is_action_pressed(rl, GameAction::MoveDown)
                && !key_bindings.is_action_pressed(rl, GameAction::MoveLeft);

        self.has_back_been_pressed = key_bindings.is_action_pressed(rl, GameAction::Cancel);
        self.has_menu_been_pressed = key_bindings.is_action_pressed(rl, GameAction::Menu);
        self.has_confirmation_been_pressed = key_bindings.is_action_pressed(rl, GameAction::Confirm);
        self.has_attack_key_been_pressed = key_bindings.is_action_pressed(rl, GameAction::Attack);
        self.has_backspace_been_pressed = key_bindings.is_action_pressed(rl, GameAction::Backspace);

        self.direction_up.update(rl, time_since_last_update, &key_bindings);
        self.direction_right.update(rl, time_since_last_update, &key_bindings);
        self.direction_down.update(rl, time_since_last_update, &key_bindings);
        self.direction_left.update(rl, time_since_last_update, &key_bindings);

        self.currently_pressed_character = rl.get_char_pressed();
        self.currently_pressed_key = rl.get_key_pressed();
    }

    pub fn on_world_changed(&mut self) {
        self.discard_direction_events_until_next_arrow_key_is_pressed = true;
    }

    pub fn direction_based_on_current_keys(&self, current: Direction) -> Direction {
        if self.discard_direction_events_until_next_arrow_key_is_pressed {
            return Direction::Unknown;
        }

        let direction_from_new_keys = Direction::from_data(
            !matches!(current, Direction::Up) && self.direction_up.is_down,
            !matches!(current, Direction::Right) && self.direction_right.is_down,
            !matches!(current, Direction::Down) && self.direction_down.is_down,
            !matches!(current, Direction::Left) && self.direction_left.is_down,
        );
        if direction_from_new_keys != current && direction_from_new_keys != Direction::Unknown {
            return direction_from_new_keys;
        }

        Direction::from_data(
            self.direction_up.is_down,
            self.direction_right.is_down,
            self.direction_down.is_down,
            self.direction_left.is_down,
        )
    }

    pub fn is_any_arrow_key_down(&self) -> bool {
        self.direction_up.is_down
            || self.direction_right.is_down
            || self.direction_down.is_down
            || self.direction_left.is_down
    }
}

pub struct HoldableKey {
    action: GameAction,
    time_to_next_press_event: f32,
    pub is_down: bool,
    pub is_pressed: bool,
}

impl HoldableKey {
    const fn new(action: GameAction) -> Self {
        Self {
            action,
            time_to_next_press_event: 0.0,
            is_down: false,
            is_pressed: false,
        }
    }

    fn update(&mut self, rl: &RaylibHandle, time_since_last_update: f32, key_bindings: &KeyBindings) {
        self.is_down = key_bindings.is_action_down(rl, self.action);
        self.is_pressed = key_bindings.is_action_pressed(rl, self.action);

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
