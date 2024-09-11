use raylib::color::Color;
use crate::{game_engine::keyboard_events_provider::KeyboardEventsProvider, hstack, lang::localizable::LocalizableText, spacing, text, ui::components::{scaffold_background_backdrop, Spacing, TextStyle, View}, vstack, zstack};

pub struct TextInput {
    pub title: String,
    pub text: String,
    pub is_confirmed: bool,
    pub is_cancelled: bool,
    pub cursor_position: usize, 
    pub cursor_visible: bool,
    time_since_shown: f32,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            text: String::new(),
            is_confirmed: false,
            is_cancelled: false,
            cursor_position: 0,
            cursor_visible: true,
            time_since_shown: 0.0,
        }
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) {
        let did_not_press_character = keyboard.currently_pressed_character.is_none();

        self.time_since_shown += time_since_last_update;
        self.cursor_visible = (self.time_since_shown * 2.0).floor() as u32 % 2 == 0;

        if let Some(character) = keyboard.currently_pressed_character {
            if self.cursor_position <= self.text.len() {
                self.text.insert(self.cursor_position, character);
                self.cursor_position += 1; 
            }
        }

        if keyboard.has_menu_been_pressed {
            self.is_confirmed = true;
        }
        if keyboard.has_backspace_been_pressed && self.cursor_position > 0 {
            self.text.remove(self.cursor_position - 1);
            self.cursor_position -= 1; 
        }
        if keyboard.has_back_been_pressed {
            self.is_cancelled = true;
        }
        if keyboard.direction_left.is_pressed && self.cursor_position > 0 && did_not_press_character {
            self.cursor_position -= 1;
        }
        if keyboard.direction_right.is_pressed && self.cursor_position < self.text.len() && did_not_press_character {
            self.cursor_position += 1;
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
        self.cursor_position = 0; 
        self.cursor_visible = true;
        self.time_since_shown = 0.0;
    }

    pub fn ui(&self) -> View {
        let (before_cursor, after_cursor) = self.text.split_at(self.cursor_position);        
        let cursor_indicator = if self.cursor_visible { "|" } else { "" };

        scaffold_background_backdrop(
            true, 
            Color::BLACK,
            vstack!(
                Spacing::XL,
                text!(TextStyle::Title, self.title.clone()),
                zstack!(
                    Spacing::Zero,
                    Color::BLACK.alpha(0.0),
                    hstack!(
                        Spacing::Zero,
                        text!(TextStyle::Regular, before_cursor.to_string()),
                        text!(TextStyle::Regular, cursor_indicator.to_string()),
                        text!(TextStyle::Regular, after_cursor.to_string())
                    ),
                    vstack!(
                        Spacing::Zero,
                        spacing!(Spacing::SM),
                        text!(TextStyle::Regular, "___________________________________".to_string())
                    )
                ),
                text!(TextStyle::Regular, "text_input.hint".localized())
            )
        )
    }
}
