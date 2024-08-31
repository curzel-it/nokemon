use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}}, spacing, text, ui::ui::{scaffold, Spacing, TextStyle, View}, utils::{rect::Rect, vector::Vector2d}, vstack, worlds::utils::list_worlds_with_none};

use super::map_editor::MapEditor;

pub type MenuUpdate = (bool, Vec<WorldStateUpdate>);

#[derive(Debug)]
pub struct GameMenu {
    state: MenuState,
    map_editor: MapEditor,
    pub selected_index: usize,
    items: Vec<MenuItem>
}

#[derive(Debug)]
enum MenuState {
    Closed,
    Open,
    MapEditor,
    PlaceItem,
}

#[derive(Debug, Copy, Clone)]
enum MenuItem {
    Save,
    MapEditor,
    Exit,
}

impl MenuItem {
    fn title(&self) -> String {
        match self {
            MenuItem::Save => "Save Game".to_string(),
            MenuItem::MapEditor => "Map Editor".to_string(),
            MenuItem::Exit => "Save & Exit".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum EntityOption {
    Remove,
}

impl EntityOption {
    fn title(&self) -> String {
        match self {
            EntityOption::Remove => "Remove".to_string(),
        }
    }
}

impl GameMenu {
    pub fn new() -> Self {
        Self {
            state: MenuState::Closed,
            map_editor: MapEditor::new(),
            selected_index: 0,
            items: vec![
                MenuItem::Save,
                MenuItem::Exit,
            ]
        }
    }

    pub fn set_creative_mode(&mut self, creative_mode: bool) {
        if creative_mode {
            self.items.insert(1, MenuItem::MapEditor);
        }
    }

    pub fn is_open(&self) -> bool {
        !matches!(&self.state, MenuState::Closed)
    }

    pub fn update(&mut self, camera_vieport: &Rect, keyboard: &KeyboardEventsProvider) -> MenuUpdate {
        let updates = match self.state {
            MenuState::Closed => self.update_from_close(keyboard),
            MenuState::Open => self.update_from_open(keyboard),
            MenuState::MapEditor => self.update_from_map_editor(camera_vieport, keyboard),
            MenuState::PlaceItem => self.update_from_place_item(camera_vieport, keyboard),
        };
        (self.is_open(), updates)
    }
}

impl GameMenu {
    fn update_from_close(&mut self, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.has_menu_been_pressed {
            self.state = MenuState::Open;
            self.map_editor.worlds = list_worlds_with_none();
        }
        vec![]
    }

    fn update_from_open(&mut self, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.has_back_been_pressed {
            self.state = MenuState::Closed;
        }
        if keyboard.direction_up.is_pressed {
            if self.selected_index == 0 {
                self.selected_index = self.items.len() - 1;
            } else if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }
        if keyboard.direction_down.is_pressed {
            if self.selected_index < self.items.len() - 1 {
                self.selected_index += 1;
            } else if keyboard.direction_down.is_pressed && self.selected_index == self.items.len() - 1 {
                self.selected_index = 0;
            }
        }
        if keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed {
            if let Some(updates) = self.handle_selection_from_open() {
                return updates;
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
    
    fn handle_selection_from_open(&mut self) -> Option<Vec<WorldStateUpdate>> {
        match self.items[self.selected_index] {
            MenuItem::MapEditor => {
                self.state = MenuState::MapEditor;
            },
            MenuItem::Save => {
                return Some(vec![WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)]);
            },
            MenuItem::Exit => {
                return Some(vec![
                    WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame),
                    WorldStateUpdate::EngineUpdate(EngineStateUpdate::Exit)
                ]);
            },
        }
        None
    }
}

impl GameMenu {
    pub fn ui(&self, camera_offset: &Vector2d) -> View {
        match self.state {
            MenuState::Closed => spacing!(Spacing::Zero),
            MenuState::Open => self.menu_ui(),
            MenuState::MapEditor => self.map_editor.ui(camera_offset),
            MenuState::PlaceItem => self.map_editor.ui(camera_offset),
        }
    }

    fn menu_ui(&self) -> View {            
        scaffold(
            vstack!(
                Spacing::XL, 
                text!(TextStyle::Title, "Game GameMenu".to_string()),
                View::VStack {                        
                    spacing: Spacing::LG,
                    children: self.items.iter().enumerate().map(|(index, item)| {
                        if index == self.selected_index {
                            text!(TextStyle::Selected, format!(" > {}", item.title()))
                        } else {
                            text!(TextStyle::Regular, format!(" {}", item.title()))
                        }                            
                    }).collect()
                },
                text!(TextStyle::Caption, "Thanks for playing. @HiddenMugs".to_string())
            )
        )
    }
}