use crate::{lang::localizable::LocalizableText, text, ui::components::{empty_view, Typography, View}};

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
        if self.is_open {
            text!(Typography::Title, "death_screen.title".localized())
        } else {
            empty_view()
        }
    }
}