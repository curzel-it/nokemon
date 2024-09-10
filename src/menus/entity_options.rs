
use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}}, lang::localizable::LocalizableText, ui::components::View};

use super::{menu::{Menu, MenuItem, MenuUpdate}, text_input::TextInput};

#[derive(Debug, Copy, Clone)]
pub enum EntityOptionMenuItem {
    Remove,
    Rename,
}

impl MenuItem for EntityOptionMenuItem {
    fn title(&self) -> String {
        match self {
            EntityOptionMenuItem::Remove => "entity.menu.remove".localized(),
            EntityOptionMenuItem::Rename => "entity.menu.rename".localized(),
        }
    }
}

pub struct EntityOptionsMenu {
    entity_name: String,
    entity_id: u32,
    menu: Menu<EntityOptionMenuItem>,
    is_renaming: bool,
    text_input: TextInput,
}

impl EntityOptionsMenu {
    pub fn new() -> Self {
        Self {
            entity_name: "".to_owned(),
            entity_id: 0,
            menu: Menu::new("entity.menu.title".localized(), vec![]),
            is_renaming: false,
            text_input: TextInput::new(),
        }
    }

    pub fn show(&mut self, name: &str, id: &u32) {
        self.entity_name = name.to_owned();
        self.entity_id = *id;
        self.menu.title = name.to_owned();
        self.menu.items = vec![
            EntityOptionMenuItem::Rename,
            EntityOptionMenuItem::Remove,
        ];
        self.menu.show();
        self.is_renaming = false;
    }

    pub fn is_open(&self) -> bool {
        self.menu.is_open()
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        if self.is_renaming {
            self.text_input.update(keyboard);

            if self.text_input.did_confirm() {
                let new_name = self.text_input.text().trim().to_owned();
                self.menu.close();
                self.is_renaming = false;
                self.text_input.clear();

                return (false, vec![
                    WorldStateUpdate::RenameEntity(self.entity_id, new_name),
                    WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)
                ]);
            } else if self.text_input.did_cancel() {
                self.is_renaming = false;
                self.text_input.clear();
            }

            return (self.menu.is_open, vec![]);
        }

        self.menu.update(keyboard, time_since_last_update);

        if self.is_open() && self.menu.selection_has_been_confirmed {
            let updates = match self.menu.selected_item() {
                EntityOptionMenuItem::Remove => {
                    self.menu.clear_selection();
                    self.menu.close();
                    vec![WorldStateUpdate::RemoveEntity(self.entity_id)]
                },
                EntityOptionMenuItem::Rename => {
                    self.menu.clear_selection();
                    self.ask_for_new_name();
                    vec![]
                },
            };
            return (self.menu.is_open, updates);
        }

        (self.menu.is_open, vec![])
    }

    pub fn ui(&self) -> View {
        if self.is_renaming {
            self.text_input.ui()
        } else {
            self.menu.ui()
        }
    }

    fn ask_for_new_name(&mut self) {
        self.is_renaming = true;
        self.text_input.clear();
        self.text_input.title = "entity.menu.rename_title".localized();
    }
}