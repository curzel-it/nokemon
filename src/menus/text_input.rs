use crate::{game_engine::keyboard_events_provider::KeyboardEventsProvider, text, ui::components::{Spacing, TextStyle, View}, vstack};

pub struct TextInput {
    pub text: String,
    pub is_confirmed: bool,
    pub is_cancelled: bool,
}

impl TextInput {
    // Initialize a new TextInput struct with empty state
    pub fn new() -> Self {
        Self {
            text: String::new(),
            is_confirmed: false,
            is_cancelled: false,
        }
    }

    // Show the TextInput with the given title
    pub fn show(&mut self, title: &str, keyboard: &KeyboardEventsProvider) {
        // Update the text based on the current pressed character
        if let Some(character) = keyboard.currently_pressed_character {
            self.text.push(character);
        }

        // Check if confirmation key (e.g., Enter) was pressed
        if keyboard.has_confirmation_been_pressed {
            self.is_confirmed = true;
        }

        // Check if backspace was pressed to remove the last character
        if keyboard.has_backspace_been_pressed {
            self.text.pop();
        }

        // Check if the back or cancel key was pressed (e.g., Escape)
        if keyboard.has_back_been_pressed {
            self.is_cancelled = true;
        }
    }

    // Returns true if the user confirmed the input
    pub fn did_confirm(&self) -> bool {
        self.is_confirmed
    }

    // Returns true if the user cancelled the input
    pub fn did_cancel(&self) -> bool {
        self.is_cancelled
    }

    // Returns the current text being input
    pub fn text(&self) -> &str {
        &self.text
    }

    // Builds the UI view for the text input
    pub fn ui(&self, title: &str) -> View {
        vstack!(
            Spacing::MD,
            text!(TextStyle::Title, title.to_string()),     // Title of the text input
            text!(TextStyle::Regular, self.text.clone())     // Display current text
        )
    }
}
