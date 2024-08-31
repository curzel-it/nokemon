use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}}, spacing, ui::ui::{Spacing, View}, utils::{rect::Rect, vector::Vector2d}, worlds::utils::list_worlds_with_none};

use super::{map_editor::MapEditor, menu::{Menu, MenuItem, MenuUpdate}};

pub struct GameMenu {
    state: MenuState,
    menu: Menu<GameMenuItem>,
    map_editor: MapEditor,
}

#[derive(Debug)]
enum MenuState {
    Closed,
    Open,
    MapEditor,
    PlaceItem,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GameMenuItem {
    Save,
    MapEditor,
    Exit,
}

impl MenuItem for GameMenuItem {
    fn title(&self) -> &str {
        match self {
            GameMenuItem::Save => "Save Game",
            GameMenuItem::MapEditor => "Map Editor",
            GameMenuItem::Exit => "Save & Exit",
        }
    }
}

impl GameMenu {
    pub fn new() -> Self {
        let menu_items = vec![
            GameMenuItem::Save,
            GameMenuItem::Exit,
        ];

        let menu = Menu::new("Game Menu", menu_items, Box::new(GameMenu::on_menu_item_selected));

        Self {
            state: MenuState::Closed,
            menu,
            map_editor: MapEditor::new(),
        }
    }

    fn on_menu_item_selected(item: GameMenuItem) -> (bool, Vec<WorldStateUpdate>) {
        match item {
            GameMenuItem::Save => {
                (false, vec![WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)])
            }
            GameMenuItem::MapEditor => {
                (true, vec![])
            }
            GameMenuItem::Exit => {
                (
                    false,
                    vec![
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame),
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::Exit),
                    ],
                )
            }
        }
    }

    pub fn set_creative_mode(&mut self, creative_mode: bool) {
        if creative_mode {
            self.menu.items.insert(1, GameMenuItem::MapEditor);
        }
    }

    pub fn is_open(&self) -> bool {
        !matches!(self.state, MenuState::Closed)
    }

    pub fn update(&mut self, camera_vieport: &Rect, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        let updates = match self.state {
            MenuState::Closed => self.update_from_close(keyboard),
            MenuState::Open => self.update_from_open(keyboard, time_since_last_update),
            MenuState::MapEditor => self.update_from_map_editor(camera_vieport, keyboard),
            MenuState::PlaceItem => self.update_from_place_item(camera_vieport, keyboard),
        };
        (self.is_open(), updates)
    }

    fn update_from_close(&mut self, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.has_menu_been_pressed {
            self.state = MenuState::Open;
            self.map_editor.worlds = list_worlds_with_none();
            self.menu.show(); 
        }
        vec![]
    }

    fn update_from_open(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let (is_open, updates) = self.menu.update(keyboard, time_since_last_update);
        if !is_open {
            self.state = MenuState::Closed;
            return updates
        }
        if keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed {
            if matches!(self.menu.selected_item(), GameMenuItem::MapEditor) { 
                self.state = MenuState::MapEditor 
            }
        } 
        vec![]
    }

    fn update_from_map_editor(&mut self, camera_vieport: &Rect, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.has_back_been_pressed {
            self.state = MenuState::Open;
        }
        self.map_editor.update(camera_vieport, keyboard);

        if self.map_editor.is_placing_item() {
            self.state = MenuState::PlaceItem;
        }
        vec![]
    }

    fn update_from_place_item(&mut self, camera_vieport: &Rect, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.has_back_been_pressed {
            self.state = MenuState::MapEditor;
        }
        self.map_editor.update(camera_vieport, keyboard)
    }

    pub fn ui(&self, camera_offset: &Vector2d) -> View {
        match self.state {
            MenuState::Closed => spacing!(Spacing::Zero),
            MenuState::Open => self.menu.ui(),
            MenuState::MapEditor | MenuState::PlaceItem => self.map_editor.ui(camera_offset),
        }
    }
}
