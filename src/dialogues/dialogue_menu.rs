use raylib::prelude::*;

use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}}, hstack, menus::menu::{Menu, MenuItem}, spacing, text, ui::components::{empty_view, scaffold_background, with_fixed_size, RenderingConfig, Spacing, TextStyle, View}, utils::{animator::Animator, vector::Vector2d}, vstack};

use super::tree::{dialogue_by_id, Dialogue};

const LINES_TO_DISPLAY: i32 = 2;

pub struct DialogueMenu {
    is_open: bool,
    pub npc_id: u32,
    pub dialogue: Dialogue,
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
        let mut options_menu = Menu::empty();
        options_menu.uses_backdrop = false;

        Self {
            is_open: false,
            npc_id: 0,
            dialogue: Dialogue::empty(),
            lines: vec![],
            current_line: 0,
            time_since_last_closed: 1.0,
            text_animator: Animator::new(),
            width: 0.0,
            height: 0.0,
            options_submenu: options_menu,
        }
    }

    pub fn show(&mut self, npc_id: u32, dialogue: Dialogue, config: &RenderingConfig) {
        if self.time_since_last_closed >= 0.5 {
            self.is_open = true;
            self.npc_id = npc_id;
            self.dialogue = dialogue;
            self.current_line = 0;
            self.setup(config);
            self.text_animator.animate(0.0, 1.0, 0.3);
        } else {
            println!("Dialog request throttled")
        }
    }

    fn setup(&mut self, config: &RenderingConfig) {
        let style = TextStyle::Regular;
        let font = config.font(&style);
        let font_size = config.scaled_font_size(&style);
        let font_spacing = config.scaled_font_spacing(&style);
        let text = self.dialogue.localized_text();
        let max_width = config.rendering_scale * 340.0;
        self.width = (config.canvas_size.x - Spacing::XL.value(config) * 2.0).min(max_width);
        self.height = font.measure_text("measure me", font_size, font_spacing).y;
        self.lines = self.split_dialogue_into_lines(&text, font_size, font_spacing, font);

        self.options_submenu.close();
        self.options_submenu.items = self.dialogue.localized_options().iter()
            .map(|option| DialogueOptionMenuItem::Value(option.clone()))
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
                if (self.current_line as i32) < (self.lines.len() as i32) - LINES_TO_DISPLAY {
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
            let is_last_line = !self.lines.is_empty() && (self.current_line as i32) >= (self.lines.len() as i32 - LINES_TO_DISPLAY);
            let has_options = !self.options_submenu.items.is_empty();
            
            if has_options && is_last_line {
                self.options_submenu.show();
            }
        } else {
            self.options_submenu.update(keyboard, time_since_last_update);
        }

        if self.options_submenu.selection_has_been_confirmed {
            let mut updates: Vec<WorldStateUpdate> = vec![];
            let (answer_text, answer) = self.dialogue.options[self.options_submenu.selected_index];
            let stops = answer_text == 0;
            println!("Confirmed selection index {}, links to {}", self.options_submenu.selected_index, answer);
            
            if let Some(next_dialogue) = dialogue_by_id(answer) {                
                println!("Found new dialogue: {:#?}", next_dialogue);
                let update_dialogue = WorldStateUpdate::ProgressConversation(self.npc_id, next_dialogue.clone());
                updates.push(update_dialogue);

                if !stops {
                    println!("Pushing dialogue");
                    self.time_since_last_closed = 1.0;
                    let show_next_dialogue = WorldStateUpdate::EngineUpdate(EngineStateUpdate::ShowDialogue(self.npc_id, next_dialogue));
                    updates.push(show_next_dialogue);
                } else {
                    println!("Got dialogue stop");
                }
            } else {
                println!("Broken dialogue link! {} -> {}", self.dialogue.id, answer);
            }
            self.options_submenu.clear_selection();
            return (self.is_open, updates);
        }

        (self.is_open, vec![])
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn ui(&self) -> View {
        if !self.is_open {
            return spacing!(Spacing::Zero)
        }
        let current_dialogue = &self.lines[self.current_line];

        // Get the animated text length for the current line
        let animated_text_length = (current_dialogue.len() as f32 * self.text_animator.current_value).round() as usize;
        let animated_text = &current_dialogue[..animated_text_length.min(current_dialogue.len())];

        // Ensure we are not going out of bounds when getting lines
        let start_index = if self.current_line + 1 >= self.lines.len() {
            self.current_line
        } else {
            self.current_line
        };

        // Get the lines to display
        let first_line = &self.lines[start_index];
        let second_line = if start_index + 1 < self.lines.len() {
            &self.lines[start_index + 1]
        } else {
            ""
        };

        // Animate the second line only
        let animated_line_to_display = if self.current_line + 1 < self.lines.len() {
            &second_line[..animated_text_length.min(second_line.len())]
        } else {
            animated_text
        };

        let has_more_lines = start_index as i32 + LINES_TO_DISPLAY - 1 < self.lines.len() as i32 - 1;
        let (spacing, next_icon) = if has_more_lines {
            (Spacing::MD, ">>")
        } else {
            (Spacing::Zero, "")
        };

        vstack!(
            Spacing::Zero,
            self.options_submenu.ui(),
            scaffold_background(
                Color::BLACK,
                vstack!(
                    Spacing::SM,
                    // First line (static)
                    with_fixed_size(
                        Vector2d::new(self.width, self.height),
                        text!(TextStyle::Regular, first_line.to_string())
                    ),
                    // Second line (animated)
                    hstack!(
                        spacing,
                        with_fixed_size(
                            Vector2d::new(self.width, self.height),
                            text!(TextStyle::Regular, animated_line_to_display.to_string())
                        ),
                        text!(TextStyle::Bold, next_icon.to_string())
                    )
                ),
            )
        )
    }
    
}
