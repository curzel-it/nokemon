use raylib::color::Color;
use crate::{game_engine::{entity::HouseholdObject, keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate, stockable::Stockable}, lang::localizable::LocalizableText, text, ui::components::{scaffold_background_backdrop, GridSpacing, Spacing, TextStyle, View}};

#[derive(Debug)]
pub struct Inventory {
    pub stock: Vec<Stockable>,
    state: InventoryState,
    columns: usize,
}

#[derive(Debug)]
enum InventoryState {
    SelectingItem(usize),
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            stock: vec![
                Stockable::HouseholdObject(HouseholdObject::SeatBrown),
                Stockable::HouseholdObject(HouseholdObject::SeatGreen),
                Stockable::HouseholdObject(HouseholdObject::SeatOrange),
                Stockable::HouseholdObject(HouseholdObject::SeatPink),
            ],
            state: InventoryState::SelectingItem(0),
            columns: 8,
        }
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        match self.state {
            InventoryState::SelectingItem(selected_index) => {
                self.update_item_selection(selected_index, keyboard)
            },
        }
    }

    fn update_item_selection(&mut self, selected_index: usize, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.direction_up.is_pressed && selected_index >= self.columns {
            self.state = InventoryState::SelectingItem(selected_index - self.columns);            
        }
        if keyboard.direction_right.is_pressed && selected_index < self.stock.len() - 1 {
            self.state = InventoryState::SelectingItem(selected_index + 1);
        }
        if keyboard.direction_down.is_pressed && selected_index < self.stock.len() - self.columns {
            self.state = InventoryState::SelectingItem(selected_index + self.columns);
        } 
        if keyboard.direction_left.is_pressed && selected_index > 0 {
            self.state = InventoryState::SelectingItem(selected_index - 1);
        }
        if keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed {
            self.handle_selection(); 
        }
        vec![]
    }

    fn handle_selection(&mut self) {
        // ...
    }
}

impl Inventory {
    pub fn ui(&self) -> View {
        scaffold_background_backdrop(
            true, 
            Color::BLACK,
            match self.state {
                InventoryState::SelectingItem(selected_index) => self.regular_ui(selected_index),                
            }
        )
    }
    
    fn regular_ui(&self, selected_item_index: usize) -> View {
        let ui_elements = vec![
            text!(TextStyle::Title, "inventory.title".localized()),
            text!(TextStyle::Regular, "inventory.subtitle".localized()),
            View::VGrid {
                spacing: GridSpacing::sm(),
                columns: self.columns,
                children: self.stock.iter().enumerate().map(|(index, item)| {
                    item.ui(index, selected_item_index)
                }).collect()
            },
        ];

        View::VStack { spacing: Spacing::LG, children: ui_elements }
    }
}