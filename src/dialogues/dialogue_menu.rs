use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}}, hstack, menus::menu::{Menu, MenuItem}, spacing, text, ui::components::{empty_view, scaffold_background, with_fixed_size, RenderingConfig, Spacing, TextStyle, View}, utils::{animator::Animator, vector::Vector2d}, vstack};

use super::tree::{dialogue_by_id, Dialogue};

pub struct DialogueMenu {
    pub npc_id: u32,
    pub dialogue: Dialogue,
    time_since_last_closed: f32,
    text_animator: Animator,
    menu: Menu<DialogueAnswerItem>,
}

#[derive(Clone)]
enum DialogueAnswerItem {
    Value(String)
}

impl MenuItem for DialogueAnswerItem {
    fn title(&self) -> String {
        match self {
            DialogueAnswerItem::Value(text) => text.clone()
        }
    }
}

impl DialogueMenu {
    pub fn new() -> Self {
        let mut options_menu = Menu::empty();
        options_menu.uses_backdrop = false;

        Self {
            npc_id: 0,
            dialogue: Dialogue::empty(),
            time_since_last_closed: 1.0,
            text_animator: Animator::new(),
            menu: options_menu,
        }
    }

    pub fn show(&mut self, npc_id: u32, dialogue: Dialogue) {
        if self.time_since_last_closed >= 0.5 {
            self.show_now(npc_id, dialogue);
        }
    }

    fn show_now(&mut self, npc_id: u32, dialogue: Dialogue) {
        self.npc_id = npc_id;
        self.dialogue = dialogue;       
        self.text_animator.animate(0.0, 1.0, 0.3);
        self.time_since_last_closed = 0.0;
        
        self.menu.text = Some(self.dialogue.localized_text());
        self.menu.items = self.dialogue.localized_options().iter()
            .map(|option| DialogueAnswerItem::Value(option.clone()))
            .collect();

        self.menu.show();
    }

    pub fn update(
        &mut self,
        keyboard: &KeyboardEventsProvider,
        time_since_last_update: f32,
    ) -> (bool, Vec<WorldStateUpdate>) {
        self.text_animator.update(time_since_last_update);

        if !self.menu.is_open() {
            self.time_since_last_closed += time_since_last_update;
        }

        if self.menu.is_open() {
            self.menu.update(keyboard, time_since_last_update);
        }

        if self.menu.selection_has_been_confirmed {
            let (answer_text, answer) = self.dialogue.options[self.menu.selected_index];
            let stops = answer_text == 0;
            let updates = self.handle_answer(stops, answer);
            return (self.menu.is_open(), updates);
        }

        (self.menu.is_open(), vec![])
    }

    fn handle_answer(&mut self, stops: bool, answer: u32) -> Vec<WorldStateUpdate> {
        let mut updates: Vec<WorldStateUpdate> = vec![];
        
        if let Some(next_dialogue) = dialogue_by_id(answer) {                
            self.menu.clear_selection();
            let update_dialogue = WorldStateUpdate::ProgressConversation(self.npc_id, next_dialogue.clone());
            updates.push(update_dialogue);

            if stops {
                self.dialogue = Dialogue::empty();
                self.menu.close();
            } else {
                self.show_now(self.npc_id, next_dialogue);
            }
        } 
        updates
    }

    pub fn is_open(&self) -> bool {
        self.menu.is_open()
    }

    pub fn ui(&self) -> View {
        self.menu.ui()
    }
    
}
