use raylib::color::Color;

use crate::{game_engine::keyboard_events_provider::KeyboardEventsProvider, text, ui::components::{scaffold_background_backdrop, Spacing, TextStyle, View}, vstack, zstack};

pub struct TextInput {
    pub title: String,
    pub text: String,
    pub is_confirmed: bool,
    pub is_cancelled: bool,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            text: String::new(),
            is_confirmed: false,
            is_cancelled: false,
        }
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider) {
        if let Some(character) = keyboard.currently_pressed_character {
            self.text.push(character);
        }
        if keyboard.has_confirmation_been_pressed {
            self.is_confirmed = true;
        }
        if keyboard.has_backspace_been_pressed {
            self.text.pop();
        }
        if keyboard.has_back_been_pressed {
            self.is_cancelled = true;
        }
    }

    pub fn did_confirm(&self) -> bool {
        self.is_confirmed
    }

    pub fn did_cancel(&self) -> bool {
        self.is_cancelled
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn clear(&mut self) {
        self.text = String::new();
        self.title = String::new();
        self.is_confirmed = false;
        self.is_cancelled = false;
    }

    pub fn ui(&self) -> View {
        scaffold_background_backdrop(
            true, 
            Color::BLACK,
            vstack!(
                Spacing::MD,
                text!(TextStyle::Title, self.title.clone()),
                text!(TextStyle::Regular, self.text.clone())
            )
        )
    }
}
