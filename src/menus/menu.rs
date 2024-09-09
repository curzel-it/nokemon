use raylib::color::Color;

use crate::constants::{MENU_CLOSE_TIME, MENU_OPEN_TIME};
use crate::ui::components::{empty_view, scaffold_background_backdrop};
use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, text, ui::components::{Spacing, TextStyle, View}, utils::animator::Animator, vstack};

pub struct Menu<Item: MenuItem> {
    pub title: String,
    pub text: Option<String>,
    pub is_open: bool,
    pub selected_index: usize,
    pub selection_has_been_confirmed: bool,
    pub items: Vec<Item>,
    pub animator: Animator,
    pub on_selection: OnMenuItemSelection<Item>,
    pub uses_backdrop: bool,
    pub visible_item_count: usize,
    pub scroll_offset: usize, 
}

pub trait MenuItem: Clone {
    fn title(&self) -> String;
}

pub type MenuUpdate = (bool, Vec<WorldStateUpdate>);
pub type OnMenuItemSelection<Item> = Box<dyn FnMut(Item) -> (bool, Vec<WorldStateUpdate>)>;

impl<Item: MenuItem> Menu<Item> {
    pub fn new(
        title: String, 
        items: Vec<Item>, 
        on_selection: OnMenuItemSelection<Item>
    ) -> Self {
        Self {
            title,
            text: None,
            is_open: false,
            selected_index: 0,
            selection_has_been_confirmed: false,
            items,
            animator: Animator::new(),
            on_selection,
            uses_backdrop: true,
            visible_item_count: 5,
            scroll_offset: 0, 
        }
    }

    pub fn empty() -> Self {
        Self::empty_with_title("".to_string())
    }

    pub fn empty_with_title(title: String) -> Self {
        Self::new(
            title, 
            vec![], 
            Box::new(|_| { (false, vec![]) })
        )
    }

    pub fn show(&mut self) {
        self.is_open = true;
        self.animator.animate(0.0, 1.0, MENU_OPEN_TIME)
    }

    pub fn show_no_animation(&mut self) {
        self.is_open = true;
        self.animator.current_value = 1.0;
        self.animator.is_active = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.animator.animate(1.0, 0.0, MENU_CLOSE_TIME)
    }

    pub fn selected_item(&self) -> Item {
        self.items[self.selected_index].clone()
    }

    pub fn clear_selection(&mut self) {
        self.selected_index = 0;
        self.selection_has_been_confirmed = false;
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        self.animator.update(time_since_last_update);

        if self.is_open {
            return (true, self.do_update(keyboard))
        }
        (false, vec![])
    }
}

impl<Item: MenuItem> Menu<Item> {
    fn do_update(&mut self, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.has_back_been_pressed {
            self.close();
        }
    
        let max_index = self.items.len() - 1;
        
        if keyboard.direction_up.is_pressed && self.selected_index > 0 {
            self.selected_index -= 1;

            if self.selected_index < self.scroll_offset {
                self.scroll_offset -= 1;
            }
        }
    
        if keyboard.direction_down.is_pressed && self.selected_index < max_index {
            self.selected_index += 1;

            if self.selected_index >= self.scroll_offset + self.visible_item_count {
                self.scroll_offset += 1;
            }
        }
    
        if keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed {
            return self.handle_selection();
        }
    
        vec![]
    }
    
    
    fn handle_selection(&mut self) -> Vec<WorldStateUpdate> {
        self.selection_has_been_confirmed = true;
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
            empty_view()
        }
    }

    fn menu_ui(&self) -> View {
        let start_index = self.scroll_offset;
        let end_index = (self.scroll_offset + self.visible_item_count).min(self.items.len());
    
        let visible_items: Vec<View> = self.items[start_index..end_index]
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let actual_index = start_index + i;
                if actual_index == self.selected_index {
                    text!(TextStyle::Selected, format!(" > {}", item.title()))
                } else {
                    text!(TextStyle::Regular, format!(" {}", item.title()))
                }
            })
            .collect();
    
        let mut children: Vec<View> = Vec::new();
    
        if self.scroll_offset > 0 {
            children.push(text!(TextStyle::Regular, "^".to_owned()));
        }
    
        children.extend(visible_items);
    
        if self.scroll_offset + self.visible_item_count < self.items.len() {
            children.push(text!(TextStyle::Regular, "...".to_owned()));
        }
    
        scaffold_background_backdrop(
            self.uses_backdrop,
            Color::BLACK.alpha(self.animator.current_value),
            vstack!(
                Spacing::XL, 
                if self.title.is_empty() {
                    empty_view()
                } else {
                    text!(TextStyle::Title, self.title.clone())
                },
                if let Some(text) = self.text.clone() {
                    text!(TextStyle::Regular, text)
                } else {
                    empty_view()
                },
                View::VStack {
                    spacing: Spacing::LG,
                    children
                }
            )
        )
    }
}
