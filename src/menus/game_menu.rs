use crate::{constants::WORLD_ID_NONE, game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}}, lang::localizable::LocalizableText, spacing, ui::components::{Spacing, View}, utils::{rect::Rect, vector::Vector2d}, worlds::utils::list_worlds_with_none};

use super::{inventory::Inventory, map_editor::MapEditor, menu::{Menu, MenuItem, MenuUpdate}};

pub struct GameMenu {
    pub current_world_id: u32,
    state: MenuState,
    menu: Menu<GameMenuItem>,
    inventory: Inventory,
    map_editor: MapEditor,
}

#[derive(Debug)]
enum MenuState {
    Closed,
    Open,
    Inventory,
    MapEditor,
    PlaceItem,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GameMenuItem {
    Save,
    Inventory,
    MapEditor,
    Exit,
}

impl MenuItem for GameMenuItem {
    fn title(&self) -> String {
        match self {
            GameMenuItem::Save => "game.menu.save".localized(),
            GameMenuItem::Inventory => "game.menu.inventory".localized(),
            GameMenuItem::MapEditor => "game.menu.map_editor".localized(),
            GameMenuItem::Exit => "game.menu.save_and_exit".localized(),
        }
    }
}

impl GameMenu {
    pub fn new() -> Self {
        let menu = Menu::new(
            "game.menu.title".localized(), 
            vec![
                GameMenuItem::Save,
                GameMenuItem::Inventory,
                GameMenuItem::Exit,
            ], 
            Box::new(GameMenu::on_menu_item_selected)
        );

        Self {
            current_world_id: WORLD_ID_NONE,
            state: MenuState::Closed,
            menu,
            inventory: Inventory::new(),
            map_editor: MapEditor::new(),
        }
    }

    fn on_menu_item_selected(item: GameMenuItem) -> (bool, Vec<WorldStateUpdate>) {
        match item {
            GameMenuItem::Save => {
                (false, vec![WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)])
            }
            GameMenuItem::Inventory => {
                (true, vec![])
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

    pub fn update(
        &mut self, 
        camera_vieport: &Rect, 
        keyboard: &KeyboardEventsProvider, 
        time_since_last_update: f32
    ) -> MenuUpdate {
        let updates = match self.state {
            MenuState::Closed => self.update_from_close(keyboard),
            MenuState::Open => self.update_from_open(keyboard, time_since_last_update),
            MenuState::Inventory => self.update_from_inventory(keyboard, time_since_last_update),
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
        let did_select_something = keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed;

        if !is_open {
            self.state = MenuState::Closed;
            return updates
        }
        if did_select_something {
            match self.menu.selected_item() {
                GameMenuItem::Inventory => {
                    self.state = MenuState::Inventory;
                }
                GameMenuItem::MapEditor => {
                    self.state = MenuState::MapEditor;
                    self.map_editor.current_world_id = self.current_world_id;
                }
                _ => {}
            }
        }
        updates
    }

    fn update_from_inventory(&mut self, keyboard: &KeyboardEventsProvider, _: f32) -> Vec<WorldStateUpdate> {
        if keyboard.has_back_been_pressed {
            self.state = MenuState::Open;
        }
        self.inventory.update(keyboard);
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
            MenuState::Inventory => self.inventory.ui(),
            MenuState::MapEditor | MenuState::PlaceItem => self.map_editor.ui(camera_offset),
        }
    }
}
