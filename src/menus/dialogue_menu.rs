use raylib::prelude::*;

use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, spacing, text, ui::components::{scaffold_background, RenderingConfig, Spacing, TextStyle, View}, vstack};

pub struct DialogueMenu {
    is_open: bool,
    npc_id: u32,
    dialogue: Vec<String>,
    current_line: usize,
    time_since_last_closed: f32,
}

impl DialogueMenu {
    pub fn new() -> Self {
        Self {
            is_open: false,
            npc_id: 0,
            dialogue: vec![],
            current_line: 0,
            time_since_last_closed: 1.0,
        }
    }

    pub fn showfrfr(&mut self, npc_id: u32, dialogue_id: u32, config: &RenderingConfig) {
        if self.time_since_last_closed >= 0.5 {
            self.is_open = true;
            self.npc_id = npc_id;
            self.dialogue = self.load_dialog(dialogue_id, config);
            self.current_line = 0;
        }
    }

    fn load_dialog(&self, _dialogue_id: u32, config: &RenderingConfig) -> Vec<String> {
        let style = TextStyle::Regular;
        let max_width = (config.canvas_size.x - Spacing::LG.value(config) * 2.0).min(600.0);
        let font = config.font(&style);
        let font_size = config.scaled_font_size(&style);
        let font_spacing = config.scaled_font_spacing(&style);
        
        let dialogue = "Hello world! This is a dialog demo. Does it fit? Idk! This is a long line just to see what happens... Or doesn't happen lol";
        self.split_dialogue_into_lines(dialogue, max_width, font_size, font_spacing, font)
    }

    fn split_dialogue_into_lines(&self, dialogue: &str, max_width: f32, font_size: f32, font_spacing: f32, font: &Font) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in dialogue.split_whitespace() {
            let potential_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            let line_width = font.measure_text(&potential_line, font_size, font_spacing).x;

            if line_width <= max_width {
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

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> (bool, Vec<WorldStateUpdate>) {
        if self.is_open {
            if keyboard.has_confirmation_been_pressed {
                if self.current_line < self.dialogue.len() - 1 {
                    self.current_line += 1;
                } else {
                    self.time_since_last_closed = 0.0;
                    self.is_open = false;
                }
            }
        } else {
            self.time_since_last_closed += time_since_last_update;
        }

        (self.is_open, vec![])
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn ui(&self) -> View {
        if self.is_open {
            let current_dialogue = &self.dialogue[self.current_line];
            scaffold_background(
                Color::BLACK,
                text!(TextStyle::Regular, current_dialogue.clone())
            )
        } else {
            spacing!(Spacing::Zero)
        }
   }   
}
