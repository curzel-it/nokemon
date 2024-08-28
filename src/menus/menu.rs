use crate::{game_engine::{keyboard_events_provider::KeyboardState, state_updates::{EngineStateUpdate, WorldStateUpdate}}, spacing, text, ui::ui::{scaffold, Spacing, TextStyle, View}, utils::{rect::Rect, vector::Vector2d}, vstack};

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
    PlaceItem,
    BuildingInteraction(u32),
    NpcInteraction(u32),
    EntityInteraction(u32),
}

pub struct MenuUpdateResult {
    pub game_paused: bool,
    pub state_updates: Vec<WorldStateUpdate>
}

#[derive(Debug, Copy, Clone)]
enum MenuItem {
    Save,
    MapEditor,
    Exit,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            state: MenuState::Closed,
            map_editor: MapEditor::new(),
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

    pub fn show_building_interaction(&mut self, id: &u32) {
        self.state = MenuState::BuildingInteraction(id.clone());
    }

    pub fn show_entity_interaction(&mut self, id: &u32) {
        self.state = MenuState::EntityInteraction(id.clone());
    }

    pub fn show_npc_interaction(&mut self, id: &u32) {
        self.state = MenuState::NpcInteraction(id.clone());
    }

    pub fn update(&mut self, camera_vieport: &Rect, keyboard_state: &KeyboardState) -> MenuUpdateResult {
        let updates = match self.state {
            MenuState::Closed => self.update_from_close(keyboard_state),
            MenuState::Open => self.update_from_open(keyboard_state),
            MenuState::MapEditor => self.update_from_map_editor(camera_vieport, keyboard_state),
            MenuState::PlaceItem => self.update_from_place_item(camera_vieport, keyboard_state),
            MenuState::BuildingInteraction(id) => self.close_or_remove_entity(id, keyboard_state),
            MenuState::NpcInteraction(id) => self.close_or_remove_entity(id, keyboard_state),
            MenuState::EntityInteraction(id) => self.close_or_remove_entity(id, keyboard_state),
        };
        MenuUpdateResult {
            game_paused: self.is_open(),
            state_updates: updates
        }
    }
}

impl Menu {
    fn close_or_remove_entity(&mut self, id: u32, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if keyboard_state.has_back_been_pressed {
            self.state = MenuState::Closed;
        }
        if keyboard_state.has_confirmation_been_pressed {
            self.state = MenuState::Closed;
            let remove = WorldStateUpdate::RemoveEntity(id);
            return vec![remove];
        }
        vec![]
    }

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

        if self.map_editor.is_placing_item() {
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

impl MenuItem {
    fn title(&self) -> String {
        match self {
            MenuItem::Save => "Save Game".to_string(),
            MenuItem::MapEditor => "Map Editor".to_string(),
            MenuItem::Exit => "Save & Exit".to_string(),
        }
    }
}

impl Menu {
    pub fn ui(&self, camera_offset: &Vector2d) -> View {
        match self.state {
            MenuState::Closed => spacing!(Spacing::Zero),
            MenuState::Open => self.menu_ui(),
            MenuState::MapEditor => self.map_editor.ui(camera_offset),
            MenuState::PlaceItem => self.map_editor.ui(camera_offset),
            MenuState::BuildingInteraction(u32) => self.remove_entity_ui(&u32),
            MenuState::NpcInteraction(u32) => self.remove_entity_ui(&u32),
            MenuState::EntityInteraction(u32) => self.remove_entity_ui(&u32),
        }
    }

    fn remove_entity_ui(&self, uuid: &u32) -> View {     
        scaffold(
            vstack!(
                Spacing::LG, 
                text!(TextStyle::Title, "Remove Entity?".to_string()),
                text!(TextStyle::Regular, format!("{}", uuid)),
                text!(TextStyle::Regular, "Press SPACE to remove.\nPress ESC to cancel.".to_string())
            )
        )
    }

    fn menu_ui(&self) -> View {            
        scaffold(
            vstack!(
                Spacing::XL, 
                text!(TextStyle::Title, "Game Menu".to_string()),
                View::VStack {                        
                    spacing: Spacing::LG,
                    children: self.items.iter().enumerate().map(|(index, item)| {
                        if index == self.selected_index {
                            text!(TextStyle::Bold, format!(" > {}", item.title()))
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