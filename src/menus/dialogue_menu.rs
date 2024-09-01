use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, spacing, text, ui::components::{Spacing, TextStyle, View}, vstack};

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

    pub fn show(&mut self, npc_id: u32, dialogue_id: u32) {
        if self.time_since_last_closed >= 0.5 {
            self.is_open = true;
            self.npc_id = npc_id;
            self.dialogue = self.load_dialog(dialogue_id);
            self.current_line = 0;
        }
    }

    fn load_dialog(&self, dialogue_id: u32) -> Vec<String> {
        vec![
            "Hello world!".to_string(),
            "This is a dialog demo".to_string(),
            "Does it fit?".to_string(),
            "Idk! This is a long line just to see what happens... Or doesn't happen lol".to_string(),
        ]
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
            vstack!(
                Spacing::MD,
                text!(TextStyle::Regular, current_dialogue.clone())
            )
        } else {
            spacing!(Spacing::Zero)
        }
   }   
}
