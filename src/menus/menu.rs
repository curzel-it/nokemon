use std::{borrow::Borrow, rc::Rc};
use std::cell::RefCell;

use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, spacing, text, ui::ui::{scaffold, Spacing, TextStyle, View}, utils::{animator::Animator, vector::Vector2d}, vstack};

pub struct Menu<Item: MenuItem> {
    title: String,
    is_open: bool,
    selected_index: usize,
    items: Vec<Item>,
    animator: Animator,
    on_selection: OnMenuItemSelection<Item>,
}

pub trait MenuItem: Clone {
    fn title(&self) -> &str;
}

pub type MenuUpdate = (bool, Vec<WorldStateUpdate>);
pub type OnMenuItemSelection<Item> = Box<dyn FnMut(Item) -> (bool, Vec<WorldStateUpdate>)>;

impl<Item: MenuItem> Menu<Item> {
    pub fn new(
        title: &str, 
        items: Vec<Item>, 
        on_selection: OnMenuItemSelection<Item>
    ) -> Self {
        Self {
            title: title.to_owned(),
            is_open: false,
            selected_index: 0,
            items,
            animator: Animator::new(),
            on_selection,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider) -> MenuUpdate {
        let updates = if self.is_open {
            self.update_from_open(keyboard)
        } else {
            self.update_from_close(keyboard)
        };
        (self.is_open, updates)
    }
}

impl<Item: MenuItem> Menu<Item> {
    fn update_from_close(&mut self, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.has_menu_been_pressed {
            self.is_open = true;
        }
        vec![]
    }

    fn update_from_open(&mut self, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.has_back_been_pressed {
            self.is_open = false;
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
            return self.handle_selection();
        }
        vec![]
    }
    
    fn handle_selection(&mut self) -> Vec<WorldStateUpdate> {
        let selected_item = self.items[self.selected_index].clone();
        let (is_open, updates) = (self.on_selection)(selected_item);
        self.is_open = is_open;
        updates
    }
}

impl<Item: MenuItem> Menu<Item> {
    pub fn ui(&self) -> View {
        if self.is_open {
            self.menu_ui()
        } else {
            spacing!(Spacing::Zero)
        }
    }

    fn menu_ui(&self) -> View {            
        scaffold(
            vstack!(
                Spacing::XL, 
                text!(TextStyle::Title, self.title.clone()),
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
