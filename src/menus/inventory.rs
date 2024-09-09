use raylib::color::Color;
use crate::{constants::{SPRITE_SHEET_INVENTORY, TILE_SIZE}, entities::species::{species_by_id, Species}, game_engine::{inventory::{get_inventory, INVENTORY}, keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, lang::localizable::LocalizableText, text, texture, ui::components::{scaffold_background_backdrop, GridSpacing, Spacing, TextStyle, View}, utils::{rect::Rect, vector::Vector2d}, zstack};

#[derive(Debug)]
pub struct Inventory {
    pub stock: Vec<Species>,
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
            stock: vec![],
            state: InventoryState::SelectingItem(0),
            columns: 8,
        }
    }

    pub fn setup(&mut self) {
        self.stock = get_inventory().iter().map(|species_id| species_by_id(*species_id)).collect()
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
                    self.item_ui(item, index, selected_item_index)
                }).collect()
            },
        ];

        View::VStack { spacing: Spacing::LG, children: ui_elements }
    }

    fn item_ui(&self, item: &Species, index: usize, selected_index: usize) -> View {
        let selected_size = 1.5 - 2.0 * Spacing::XS.unscaled_value() / TILE_SIZE;
        let (y, x) = item.inventory_texture_offset;
        let texture_source_rect = Rect::new(x, y, 1, 1);

        if index == selected_index {
            zstack!(
                Spacing::XS, 
                Color::YELLOW,
                texture!(
                    SPRITE_SHEET_INVENTORY, 
                    texture_source_rect, 
                    Vector2d::new(selected_size, selected_size)
                )
            )
        } else {
            texture!(
                SPRITE_SHEET_INVENTORY, 
                texture_source_rect, 
                Vector2d::new(1.5, 1.5)
            )
        }
    }
}