use std::rc::Rc;

use raylib::prelude::*;

use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}}, hstack, menus::menu::{Menu, MenuItem}, spacing, text, ui::components::{empty_view, scaffold_background, with_fixed_size, RenderingConfig, Spacing, TextStyle, View}, utils::{animator::Animator, vector::Vector2d}, vstack};

use super::tree::{next_dialogue, Dialogue};

pub struct DialogueMenu {
    is_open: bool,
    npc_id: u32,
    lines: Vec<String>,
    current_line: usize,
    time_since_last_closed: f32,
    text_animator: Animator,
    width: f32,
    height: f32,
    options_submenu: Menu<DialogueOptionMenuItem>,
}

#[derive(Clone)]
enum DialogueOptionMenuItem {
    Value(String)
}

impl MenuItem for DialogueOptionMenuItem {
    fn title(&self) -> String {
        match self {
            DialogueOptionMenuItem::Value(text) => text.clone()
        }
    }
}

impl DialogueMenu {
    pub fn new() -> Self {
        Self {
            is_open: false,
            npc_id: 0,
            lines: vec![],
            current_line: 0,
            time_since_last_closed: 1.0,
            text_animator: Animator::new(),
            width: 0.0,
            height: 0.0,
            options_submenu: Menu::empty(),
        }
    }

    pub fn show(&mut self, npc_id: u32, dialogue: Dialogue, config: &RenderingConfig) {
        if self.time_since_last_closed >= 0.5 {
            self.is_open = true;
            self.npc_id = npc_id;
            self.current_line = 0;
            self.setup(dialogue, config);
            self.text_animator.animate(0.0, 1.0, 0.3);
        }
    }

    fn setup(&mut self, dialogue: Dialogue, config: &RenderingConfig) {
        let style = TextStyle::Regular;
        let font = config.font(&style);
        let font_size = config.scaled_font_size(&style);
        let font_spacing = config.scaled_font_spacing(&style);
        let text = dialogue.localized_text();
        self.width = (config.canvas_size.x - Spacing::XL.value(config) * 2.0).min(600.0);
        self.height = font.measure_text("measure me", font_size, font_spacing).y;
        self.lines = self.split_dialogue_into_lines(&text, font_size, font_spacing, font);

        self.options_submenu.close();
        self.options_submenu.items = dialogue.options
            .iter()
            .map(|option| DialogueOptionMenuItem::Value(option.localized_text()))
            .collect();
    }

    fn split_dialogue_into_lines(
        &self,
        dialogue: &str,
        font_size: f32,
        font_spacing: f32,
        font: &Font,
    ) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in dialogue.split_whitespace() {
            let potential_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            let line_width = font.measure_text(&potential_line, font_size, font_spacing).x;

            if line_width <= self.width {
                current_line = potential_line;
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
        lines
    }

    pub fn update(
        &mut self,
        keyboard: &KeyboardEventsProvider,
        time_since_last_update: f32,
    ) -> (bool, Vec<WorldStateUpdate>) {
        self.text_animator.update(time_since_last_update);

        if self.is_open {
            if keyboard.has_confirmation_been_pressed {
                if self.current_line < self.lines.len() - 1 {
                    self.current_line += 1;
                    self.text_animator.animate(0.0, 1.0, 0.3);
                } else {
                    self.time_since_last_closed = 0.0;
                    self.is_open = false;
                }
            }
        } else {
            self.time_since_last_closed += time_since_last_update;
        }

        if !self.options_submenu.is_open() {
            let is_last_line = !self.lines.is_empty() && self.current_line == self.lines.len() - 1;
            let has_options = !self.options_submenu.items.is_empty();
            
            if has_options && is_last_line {
                self.options_submenu.show();
            }
        } else {
            self.options_submenu.update(keyboard, time_since_last_update);
        }

        if self.options_submenu.selection_has_been_confirmed {
            if let Some(next) = next_dialogue(self.npc_id, self.options_submenu.selected_index) {
                let show_next_dialogue = WorldStateUpdate::EngineUpdate(EngineStateUpdate::ShowDialogue(self.npc_id, next));
                return (self.is_open, vec![show_next_dialogue]);
            }
        }

        (self.is_open, vec![])
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn ui(&self) -> View {
        if self.is_open {
            let current_dialogue = &self.lines[self.current_line];
            let animated_text_length = (current_dialogue.len() as f32 * self.text_animator.current_value).round() as usize;
            let animated_text = &current_dialogue[..animated_text_length.min(current_dialogue.len())];
            let has_more_lines = self.current_line < self.lines.len() - 1;

            let (spacing, next_icon) = if has_more_lines {
                (Spacing::MD, ">>")
            } else {
                (Spacing::Zero, "")
            };

            vstack!(
                Spacing::Zero,
                scaffold_background(
                    Color::BLACK,
                    hstack!(
                        spacing,
                        with_fixed_size(
                            Vector2d::new(self.width, self.height),
                            text!(TextStyle::Regular, animated_text.to_string())
                        ),
                        text!(TextStyle::Bold, next_icon.to_string())
                    ),
                ),
                self.options_submenu.ui()
            )
        } else {
            spacing!(Spacing::Zero)
        }
    }
}
