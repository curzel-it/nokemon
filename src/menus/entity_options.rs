
use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, ui::ui::View};

use super::menu::{Menu, MenuItem, MenuUpdate, OnMenuItemSelection};

#[derive(Debug, Copy, Clone)]
pub enum EntityOptionMenuItem {
    Remove,
}

impl EntityOptionMenuItem {
    pub fn id(&self) -> u32 {
        match self {
            EntityOptionMenuItem::Remove => 0, // Assign ID as needed
        }
    }
}

impl MenuItem for EntityOptionMenuItem {
    fn title(&self) -> &str {
        match self {
            EntityOptionMenuItem::Remove => "Remove",
        }
    }
}

pub struct EntityOptionsMenu {
    entity_id: u32,
    menu: Menu<EntityOptionMenuItem>,
}

impl EntityOptionsMenu {
    pub fn new() -> Self {
        let items = vec![
            EntityOptionMenuItem::Remove,
        ];

        let on_selection: OnMenuItemSelection<EntityOptionMenuItem> = Box::new(|item| {
            match item {
                EntityOptionMenuItem::Remove => {
                    (false, vec![WorldStateUpdate::RemoveEntity(item.id())])
                }
            }
        });

        Self {
            entity_id: 0,
            menu: Menu::new("Entity Options", items, on_selection),
        }
    }

    pub fn show(&mut self, id: u32) {
        self.entity_id = id;
        self.menu.show()
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider) -> MenuUpdate {
        self.menu.update(keyboard)
    }

    pub fn is_open(&self) -> bool {
        self.menu.is_open()
    }

    pub fn ui(&self) -> View {
        self.menu.ui()
    }
}
