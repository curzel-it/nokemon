use crate::{
    game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}}, lang::localizable::LocalizableText, menus::menu::{Menu, MenuItem, MenuUpdate, OnMenuItemSelection}, ui::components::View
};

#[derive(Debug, Copy, Clone)]
pub enum NpcOptionsMenuItem {
    RemoveEntity(u32),
    PlayDialog(u32),
}

impl MenuItem for NpcOptionsMenuItem {
    fn title(&self) -> String {
        match self {
            NpcOptionsMenuItem::RemoveEntity(_) => "npc.menu.remove".localized(),
            NpcOptionsMenuItem::PlayDialog(_) => "npc.menu.play_dialog".localized(),
        }
    }
}

pub struct NpcOptionsMenu {
    menu: Menu<NpcOptionsMenuItem>,
}

impl NpcOptionsMenu {
    pub fn new() -> Self {
        let on_selection: OnMenuItemSelection<NpcOptionsMenuItem> = Box::new(|item| {
            match item {
                NpcOptionsMenuItem::RemoveEntity(id) => {
                    (false, vec![WorldStateUpdate::RemoveEntity(id)])
                }
                NpcOptionsMenuItem::PlayDialog(dialogue_id) => {
                    (false, vec![WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowDialogue(dialogue_id, dialogue_id),
                    )])
                }
            }
        });

        Self {
            menu: Menu::new("npc.menu.title".localized(), vec![], on_selection),
        }
    }

    pub fn show(&mut self, id: u32, dialogue_id: u32) {
        self.menu.items = vec![
            NpcOptionsMenuItem::RemoveEntity(id),
            NpcOptionsMenuItem::PlayDialog(dialogue_id),
        ];
        self.menu.show()
    }

    pub fn is_open(&self) -> bool {
        self.menu.is_open()
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        self.menu.update(keyboard, time_since_last_update)
    }

    pub fn ui(&self) -> View {
        self.menu.ui()
    }
}
