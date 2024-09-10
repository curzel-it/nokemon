
use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, lang::localizable::LocalizableText, ui::components::View};

use super::menu::{Menu, MenuItem, MenuUpdate, OnMenuItemSelection};

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
}

impl EntityOptionsMenu {
    pub fn new() -> Self {
        Self {
            entity_name: "".to_owned(),
            entity_id: 0,
            menu: Menu::new("entity.menu.title".localized(), vec![]),
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
        self.menu.show()
    }

    pub fn is_open(&self) -> bool {
        self.menu.is_open()
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        self.menu.update(keyboard, time_since_last_update);

        if self.is_open() && self.menu.selection_has_been_confirmed {
            let updates = match self.menu.selected_item() {
                EntityOptionMenuItem::Remove => {
                    self.menu.close();
                    vec![WorldStateUpdate::RemoveEntity(self.entity_id)]
                },
                EntityOptionMenuItem::Rename => vec![],
            };
            return (self.menu.is_open, updates)
        }
        (self.menu.is_open, vec![])
    }

    pub fn ui(&self) -> View {
        self.menu.ui()
    }
}
