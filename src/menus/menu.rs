use raylib::color::Color;

use crate::constants::{MENU_CLOSE_TIME, MENU_OPEN_TIME};
use crate::ui::ui::scaffold_with_bg;
use crate::{game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, spacing, text, ui::ui::{Spacing, TextStyle, View}, utils::animator::Animator, vstack};

pub struct Menu<Item: MenuItem> {
    title: String,
    is_open: bool,
    selected_index: usize,
    pub items: Vec<Item>,
    animator: Animator,
    pub on_selection: OnMenuItemSelection<Item>,
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
            is_open: false,
            selected_index: 0,
            items,
            animator: Animator::new(),
            on_selection,
        }
    }

    pub fn show(&mut self) {
        self.is_open = true;
        self.animator.animate(0.0, 1.0, MENU_OPEN_TIME)
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
        self.animator.animate(1.0, 0.0, MENU_CLOSE_TIME)
    }

    pub fn selected_item(&self) -> Item {
        self.items[self.selected_index].clone()
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
        scaffold_with_bg(
            Color::BLACK.alpha(self.animator.current_value),
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
