
use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, ui::ui::View};

use super::menu::{Menu, MenuItem, MenuUpdate, OnMenuItemSelection};

#[derive(Debug, Copy, Clone)]
pub enum EntityOptionMenuItem {
    RemoveEntity(u32),
}

impl MenuItem for EntityOptionMenuItem {
    fn title(&self) -> &str {
        match self {
            EntityOptionMenuItem::RemoveEntity(_) => "Remove",
        }
    }
}

pub struct EntityOptionsMenu {
    menu: Menu<EntityOptionMenuItem>,
}

impl EntityOptionsMenu {
    pub fn new() -> Self {
        let on_selection: OnMenuItemSelection<EntityOptionMenuItem> = Box::new(|item| {
            match item {
                EntityOptionMenuItem::RemoveEntity(id) => {
                    (false, vec![WorldStateUpdate::RemoveEntity(id)])
                }
            }
        });

        Self {
            menu: Menu::new("Entity Options", vec![], on_selection),
        }
    }

    pub fn show(&mut self, id: u32) {
        self.menu.items = vec![
            EntityOptionMenuItem::RemoveEntity(id),
        ];
        self.menu.show()
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        self.menu.update(keyboard, time_since_last_update)
    }

    pub fn ui(&self) -> View {
        self.menu.ui()
    }
}
