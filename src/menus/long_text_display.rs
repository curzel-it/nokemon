use raylib::color::Color;

use crate::constants::{MENU_CLOSE_TIME, MENU_OPEN_TIME};
use crate::ui::components::empty_view;
use crate::ui::scaffold::scaffold;
use crate::{game_engine::keyboard_events_provider::KeyboardEventsProvider, text, ui::components::{Spacing, TextStyle, View}, utils::animator::Animator, vstack};

use super::menu::MENU_BORDERS_TEXTURES;

pub struct LongTextDisplay {
    pub text: String,
    pub is_open: bool,
    pub visible_line_count: usize,
    pub scroll_offset: usize,
    pub animator: Animator,
    pub uses_backdrop: bool,
    pub max_line_length: usize,
    pub lines: Vec<String>,
}

impl LongTextDisplay {
    pub fn new(max_line_length: usize, visible_line_count: usize) -> Self {
        Self {
            text: "".to_owned(),
            is_open: false,
            visible_line_count,
            scroll_offset: 0,
            animator: Animator::new(),
            uses_backdrop: true,
            max_line_length,
            lines: vec![],
        }
    }

    pub fn show(&mut self, text: String) {
        self.text = text;
        self.lines = Self::wrap_text(&self.text, self.max_line_length);
        self.is_open = true;
        self.animator.animate(0.0, 1.0, MENU_OPEN_TIME);
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.animator.animate(1.0, 0.0, MENU_CLOSE_TIME);
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> bool {
        self.animator.update(time_since_last_update);

        if self.is_open {
            if keyboard.has_back_been_pressed {
                self.close();
            }
            let max_offset = self.lines.len().saturating_sub(self.visible_line_count);

            if keyboard.direction_up.is_pressed && self.scroll_offset > 0 {
                self.scroll_offset -= 1;
            }
            if keyboard.direction_down.is_pressed && self.scroll_offset < max_offset {
                self.scroll_offset += 1;
            }
        }
        self.is_open
    }

    pub fn ui(&self) -> View {
        if self.is_open {
            let background_color = Color::BLACK.alpha(self.animator.current_value);

            scaffold(
                self.uses_backdrop,
                background_color,
                Some(MENU_BORDERS_TEXTURES),
                self.text_ui()
            )
        } else {
            empty_view()
        }
    }

    fn text_ui(&self) -> View {
        let start_index = self.scroll_offset;
        let end_index = (self.scroll_offset + self.visible_line_count).min(self.lines.len());

        let visible_lines: Vec<View> = self.lines[start_index..end_index]
            .iter()
            .map(|line| {
                text!(TextStyle::Regular, line.clone())
            })
            .collect();

        let mut children: Vec<View> = Vec::new();

        if self.scroll_offset > 0 {
            children.push(text!(TextStyle::Regular, "^".to_owned()));
        } else {
            children.push(text!(TextStyle::Regular, ":".to_owned()));
        }

        children.extend(visible_lines);

        if self.scroll_offset + self.visible_line_count < self.lines.len() {
            children.push(text!(TextStyle::Regular, "...".to_owned()));
        } else {
            children.push(text!(TextStyle::Regular, "---".to_owned()));
        }

        vstack!(
            Spacing::XL,
            View::VStack {
                spacing: Spacing::LG,
                children
            }
        )
    }

    fn wrap_text(text: &str, max_line_length: usize) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > max_line_length {
                lines.push(current_line.clone());
                current_line.clear();
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }
}
