use raylib::color::Color;
use crate::{constants::{SPRITE_SHEET_INVENTORY, TILE_SIZE, WORLD_ID_NONE}, entities::{building::BuildingType, household_objects::HouseholdObject, npc::{Npc, NpcType}, teleporter::Teleporter}, game_engine::{entity_body::EmbodiedEntity, keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate, stockable::Stockable}, lang::localizable::LocalizableText, maps::{biome_tiles::Biome, constructions_tiles::Construction}, prefabs::prefabs::new_building, spacing, text, texture, ui::components::{scaffold_background_backdrop, with_fixed_position, GridSpacing, Spacing, TextStyle, View}, utils::{ids::get_next_id, rect::Rect, vector::Vector2d}, vstack, worlds::utils::{list_worlds_with_none, world_name}, zstack};

const MAX_VISIBLE_WORLDS: usize = 4;

#[derive(Debug)]
pub struct Inventory {
    pub stock: Vec<Stockable>,
    state: InventoryState,
    sprite_sheet: u32,
    columns: usize,
    offset: usize, 
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
            sprite_sheet: SPRITE_SHEET_INVENTORY,
            columns: 8,
            offset: 0, 
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
        if keyboard.direction_down.is_pressed {
            if selected_index < self.stock.len() - self.columns {
                self.state = InventoryState::SelectingItem(selected_index + self.columns);
            }
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
                    item.ui(self.sprite_sheet, index, selected_item_index)
                }).collect()
            },
        ];

        View::VStack { spacing: Spacing::LG, children: ui_elements }
    }
}