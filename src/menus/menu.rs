use raylib::color::Color;

use crate::{constants::TILE_SIZE, game_engine::{keyboard_events_provider::KeyboardState, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, hstack, spacing, text, texture, ui::ui::{padding, with_backdrop, with_fixed_position, Spacing, TextStyle, View}, utils::{rect::Rect, vector::Vector2d}, vstack, zstack};

use super::map_editor::MapEditor;

#[derive(Debug)]
pub struct Menu {
    pub selected_index: usize,
    state: MenuState,
    items: Vec<MenuItem>,
    map_editor: MapEditor,
}

#[derive(Debug)]
enum MenuState {
    Closed,
    Open,
    MapEditor,
    PlaceItem
}

pub struct MenuUpdateResult {
    pub game_paused: bool,
    pub state_updates: Vec<WorldStateUpdate>
}

#[derive(Debug, Copy, Clone)]
enum MenuItem {
    Save,
    MapEditor,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            state: MenuState::Closed,
            map_editor: MapEditor::new(),
            items: vec![
                MenuItem::Save,
                MenuItem::MapEditor,
            ]
        }
    }

    pub fn set_creative_mode(&mut self, creative_mode: bool) {
        self.items = self.items.clone().into_iter().filter(|item| !item.is_map_editor()).collect();

        if creative_mode {
            self.items.push(MenuItem::MapEditor)
        }
    }

    pub fn is_open(&self) -> bool {
        if let MenuState::Closed = self.state {
            false
        } else {
            true
        }
    }

    pub fn update(&mut self, camera_vieport: &Rect, keyboard_state: &KeyboardState) -> MenuUpdateResult {
        let updates = match self.state {
            MenuState::Closed => self.update_from_close(keyboard_state),
            MenuState::Open => self.update_from_open(keyboard_state),
            MenuState::MapEditor => self.update_from_map_editor(camera_vieport, keyboard_state),
            MenuState::PlaceItem => self.update_from_place_item(camera_vieport, keyboard_state),
        };
        MenuUpdateResult {
            game_paused: self.is_open(),
            state_updates: updates
        }
    }
}

impl Menu {
    fn update_from_close(&mut self, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if keyboard_state.has_menu_been_pressed {
            self.state = MenuState::Open;
        }
        vec![]
    }

    fn update_from_open(&mut self, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if keyboard_state.has_back_been_pressed {
            self.state = MenuState::Closed;
        }
        if keyboard_state.has_up_been_pressed {
            if self.selected_index == 0 {
                self.selected_index = self.items.len() - 1;
            } else if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }
        if keyboard_state.has_down_been_pressed {
            if self.selected_index < self.items.len() - 1 {
                self.selected_index += 1;
            } else if keyboard_state.has_down_been_pressed && self.selected_index == self.items.len() - 1 {
                self.selected_index = 0;
            }
        }
        if keyboard_state.has_confirmation_been_pressed || keyboard_state.has_menu_been_pressed {
            if let Some(updates) = self.handle_selection_from_open() {
                return updates;
            }
        }
        if keyboard_state.has_back_been_pressed {
            self.state = MenuState::Closed;
        }
        vec![]
    }

    fn update_from_map_editor(&mut self, camera_vieport: &Rect, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if keyboard_state.has_back_been_pressed {
            self.state = MenuState::Open;
        }
        self.map_editor.update(camera_vieport, keyboard_state);

        if self.map_editor.item_being_placed.is_some() {
            self.state = MenuState::PlaceItem;
        }
        vec![]
    }

    fn update_from_place_item(&mut self, camera_vieport: &Rect, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if keyboard_state.has_back_been_pressed {
            self.state = MenuState::MapEditor;
        }
        self.map_editor.update(camera_vieport, keyboard_state)
    }
    
    fn handle_selection_from_open(&mut self) -> Option<Vec<WorldStateUpdate>> {
        match self.items[self.selected_index] {
            MenuItem::MapEditor => {
                self.state = MenuState::MapEditor;
            },
            MenuItem::Save => {
                return Some(vec![WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)]);
            },
        }
        None
    }
}

impl MenuItem {
    fn title(&self) -> String {
        match self {
            MenuItem::Save => "Save Game".to_string(),
            MenuItem::MapEditor => "MapEditor".to_string(),
        }
    }

    fn is_map_editor(&self) -> bool {
        match self {
            MenuItem::MapEditor => true,
            _ => false
        }
    }
}

impl Menu {
    pub fn ui(&self) -> View {
        match self.state {
            MenuState::Closed => spacing!(Spacing::ZERO),
            MenuState::Open => with_backdrop(self.menu_ui()),
            MenuState::MapEditor => with_backdrop(self.map_editor.ui()),
            MenuState::PlaceItem => self.map_editor.ui()
        }
    }

    fn menu_ui(&self) -> View {        
        padding(
            Spacing::LG,
            zstack!(
                Spacing::LG,
                Color::BLACK,
                vstack!(
                    Spacing::LG, 
                    text!(TextStyle::Title, "Game Menu".to_string()),
                    View::VStack {                        
                        spacing: Spacing::MD,
                        children: self.items.iter().enumerate().map(|(index, item)| {
                            if index == self.selected_index {
                                text!(TextStyle::Bold, format!("> {}", item.title()))
                            } else {
                                text!(TextStyle::Regular, item.title())
                            }                            
                        }).collect()
                    }
                )
            )
        )
    }
}