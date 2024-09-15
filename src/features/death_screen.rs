use crate::{lang::localizable::LocalizableText, text, ui::components::{Typography, View}};

pub struct DeathScreen {
    pub is_open: bool
}

impl DeathScreen {
    pub fn new() -> Self {
        Self {
            is_open: false
        }
    }

    pub fn show(&mut self) {
        self.is_open = true
    }

    pub fn ui(&self) -> View {
        text!(Typography::Title, "death_screen.title".localized())
    }
}