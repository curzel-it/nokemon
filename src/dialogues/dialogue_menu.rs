use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate, storage::{set_value_for_key, StorageKey}}, menus::menu::{Menu, MenuItem}, ui::components::View, utils::animator::Animator};

use super::dialogues::{dialogue_by_id, Dialogue};

pub struct DialogueMenu {
    pub npc_id: u32,
    pub npc_name: String,
    pub dialogue: Dialogue,
    time_since_last_closed: f32,
    text_animator: Animator,
    text: String,
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
            npc_name: "".to_string(),
            dialogue: Dialogue::empty(),
            time_since_last_closed: 1.0,
            text_animator: Animator::new(),
            text: "".to_owned(),
            menu: options_menu,
        }
    }

    pub fn show(&mut self, npc_id: u32, npc_name: String, dialogue: Dialogue) {
        if self.time_since_last_closed >= 0.5 {
            self.show_now(npc_id, npc_name, dialogue);
        }
    }

    fn show_now(&mut self, npc_id: u32, npc_name: String, dialogue: Dialogue) {
        self.npc_id = npc_id;
        self.npc_name = npc_name;
        self.dialogue = dialogue;       
        
        self.menu.title = format!("{: <45}", format!("{}:", self.npc_name));
        self.text = self.dialogue.localized_text();

        self.text_animator.animate(0.0, 1.0, self.text.len() as f32 / 80.0);
        self.time_since_last_closed = 0.0;
        
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
        
        let animated_text_length = (self.text.len() as f32 * self.text_animator.current_value).round() as usize;
        let animated_text = &self.text[..animated_text_length.min(self.text.len())];
        self.menu.text = Some(animated_text.to_owned());

        if !self.menu.is_open() {
            self.time_since_last_closed += time_since_last_update;
        }
        if self.menu.is_open() {
            self.menu.update(keyboard, time_since_last_update);
        }
        if self.menu.selection_has_been_confirmed {
            if self.text_animator.is_active {
                self.menu.is_open = true;
                self.menu.selection_has_been_confirmed = false;
                return (true, vec![]);
            } else {
                let (answer_text, answer) = self.dialogue.options[self.menu.selected_index];
                let stops = answer_text == 0;
                let updates = self.handle_answer(stops, answer);
                return (self.menu.is_open(), updates);
            }
        }

        (self.menu.is_open(), vec![])
    }

    fn handle_answer(&mut self, stops: bool, answer: u32) -> Vec<WorldStateUpdate> {
        let updates: Vec<WorldStateUpdate> = vec![];

        set_value_for_key(StorageKey::dialogue_answer(self.dialogue.id), answer);       
        self.menu.clear_selection();
        
        if let Some(next_dialogue) = dialogue_by_id(answer) {         
            // let update_dialogue = WorldStateUpdate::ProgressConversation(self.npc_id, next_dialogue.clone());
            // updates.push(update_dialogue);

            if stops {            
                self.dialogue = Dialogue::empty();
                self.menu.close();
            } else {
                self.show_now(self.npc_id, self.npc_name.clone(), next_dialogue);
            }
        }  else {
            self.dialogue = Dialogue::empty();
            self.menu.close();
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
