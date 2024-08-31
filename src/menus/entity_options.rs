use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, spacing, text, ui::ui::{scaffold, Spacing, TextStyle, View}, vstack};

use super::game_menu::MenuUpdate;

#[derive(Debug)]
pub struct EntityOptionsMenu {
    state: MenuState,
    pub selected_index: usize,
    items: Vec<MenuOption>,
}

#[derive(Debug)]
enum MenuState {
    Closed,
    Open(u32),
}

#[derive(Debug, Copy, Clone)]
enum MenuOption {
    Remove,
}

impl MenuOption {
    fn title(&self) -> String {
        match self {
            MenuOption::Remove => "Remove".to_string(),
        }
    }
}

impl EntityOptionsMenu {
    pub fn new() -> Self {
        Self {
            state: MenuState::Closed,
            selected_index: 0,
            items: vec![
                MenuOption::Remove,
            ]
        }
    }

    pub fn show(&mut self, id: &u32) {
        self.state = MenuState::Open(id.clone());
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider) -> MenuUpdate {
        let updates = match self.state {
            MenuState::Closed => vec![],
            MenuState::Open(id) => self.update_from_open(id, keyboard),
        };
        (self.is_open(), updates)
    }

    pub fn is_open(&self) -> bool {
        !matches!(&self.state, MenuState::Closed)
    }
}

impl EntityOptionsMenu {
    fn update_from_open(&mut self, id: u32, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
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
            if let Some(updates) = self.handle_selection_from_items(id) {
                return updates;
            }
        }
        vec![]
    }
    
    fn handle_selection_from_items(&mut self, id: u32) -> Option<Vec<WorldStateUpdate>> {        
        match self.items[self.selected_index] {
            MenuOption::Remove => {
                self.state = MenuState::Closed;
                return Some(vec![WorldStateUpdate::RemoveEntity(id)])
            },
        }
    }
}

impl EntityOptionsMenu {
    pub fn ui(&self) -> View {
        match self.state {
            MenuState::Closed => spacing!(Spacing::Zero),
            MenuState::Open(id) => self.menu_ui(&id),
        }
    }

    fn menu_ui(&self, id: &u32) -> View {     
        scaffold(
            vstack!(
                Spacing::LG, 
                text!(TextStyle::Title, "Entity Options".to_string()),
                text!(TextStyle::Caption, format!("Id: #{}", id)),
                View::VStack {                        
                    spacing: Spacing::LG,
                    children: self.items.iter().enumerate().map(|(index, item)| {
                        if index == self.selected_index {
                            text!(TextStyle::Selected, format!(" > {}", item.title()))
                        } else {
                            text!(TextStyle::Regular, format!(" {}", item.title()))
                        }                            
                    }).collect()
                }
            )
        )
    }
}