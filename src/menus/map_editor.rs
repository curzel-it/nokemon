use raylib::color::Color;

use crate::{constants::{INFINITE_STOCK, SPRITE_SHEET_INVENTORY, TILE_SIZE, TILE_SIZE_X1_5}, entities::building::{Building, BuildingType}, game_engine::{entity_body::EmbodiedEntity, keyboard_events_provider::KeyboardState, state_updates::WorldStateUpdate}, spacing, text, texture, ui::ui::{padding, with_fixed_position, GridSpacing, Spacing, TextStyle, View}, utils::{rect::Rect, vector::Vector2d}, vstack, worlds::constants::WORLD_ID_HOUSE_INTERIOR, zstack};

use super::inventory::Stockable;

#[derive(Debug)]
pub struct MapEditor {
    pub is_placing_item: bool,
    pub stock: Vec<MapEditorItem>,
    pub selected_index: usize,
    pub item_being_placed: Option<MapEditorItemBeingPlaced>,
    sprite_sheet: u32,
    columns: usize
}

#[derive(Debug)]
pub struct MapEditorItem {
    pub item: Stockable,
    pub stock: i32
}

#[derive(Debug, Clone, Copy)]
pub struct MapEditorItemBeingPlaced {
    pub item: Stockable,
    pub frame: Rect
}

impl MapEditor {
    pub fn new() -> Self {
        Self {
            is_placing_item: false,
            stock: Stockable::all_possible_items().into_iter()
                .map(|item| { MapEditorItem { item, stock: INFINITE_STOCK } })
                .collect(),
            selected_index: 0,
            item_being_placed: None,
            sprite_sheet: SPRITE_SHEET_INVENTORY,
            columns: 5,
        }
    }

    pub fn update(&mut self, camera_vieport: &Rect, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if self.is_placing_item {
            if keyboard_state.has_up_been_pressed {
                self.item_being_placed.as_mut().unwrap().frame.y -= TILE_SIZE;
            }
            if keyboard_state.has_right_been_pressed {
                self.item_being_placed.as_mut().unwrap().frame.x += TILE_SIZE;
            }
            if keyboard_state.has_down_been_pressed {
                self.item_being_placed.as_mut().unwrap().frame.y += TILE_SIZE;
            }
            if keyboard_state.has_left_been_pressed {
                self.item_being_placed.as_mut().unwrap().frame.x -= TILE_SIZE;
            }
            if keyboard_state.has_confirmation_been_pressed || keyboard_state.has_menu_been_pressed {
                return self.place(camera_vieport, self.item_being_placed.unwrap().item);
            }
            if keyboard_state.has_back_been_pressed {
                self.is_placing_item = false;
                self.item_being_placed = None;
            }
        } else {
            if keyboard_state.has_up_been_pressed && self.selected_index >= self.columns {
                self.selected_index -= self.columns;
            }
            if keyboard_state.has_right_been_pressed && self.selected_index < self.stock.len() - 1 {
                self.selected_index += 1;
            }
            if keyboard_state.has_down_been_pressed && self.selected_index < self.stock.len() - self.columns {
                self.selected_index += self.columns;
            }
            if keyboard_state.has_left_been_pressed && self.selected_index > 0 {
                self.selected_index -= 1;
            }
            if keyboard_state.has_confirmation_been_pressed || keyboard_state.has_menu_been_pressed {
                self.item_being_placed = Some(
                    MapEditorItemBeingPlaced {
                        item: self.stock[self.selected_index].item,
                        frame: Rect::new(
                            (camera_vieport.x / TILE_SIZE).ceil() * TILE_SIZE - camera_vieport.x,
                            (camera_vieport.y / TILE_SIZE).ceil() * TILE_SIZE - camera_vieport.y,
                            TILE_SIZE, 
                            TILE_SIZE
                        )
                    }
                );
                self.is_placing_item = true;
            }
        }
        vec![]
    }

    fn place(&self, camera_vieport: &Rect, item: Stockable) -> Vec<WorldStateUpdate> {
        let frame = self.item_being_placed.unwrap().frame;
        let row = ((camera_vieport.y + frame.y) / TILE_SIZE) as usize;
        let col = ((camera_vieport.x + frame.x) / TILE_SIZE) as usize;

        match item {
           Stockable::BiomeTile(biome) => vec![WorldStateUpdate::BiomeTileChange(row, col, biome)],
           Stockable::ConstructionTile(construction) => vec![WorldStateUpdate::ConstructionTileChange(row, col, construction)],
           Stockable::Building(building_type) => self.place_building(camera_vieport, building_type),
        }
    }

    fn place_building(&self, camera_vieport: &Rect, building_type: BuildingType) -> Vec<WorldStateUpdate> {
        let frame = self.item_being_placed.unwrap().frame;
        let mut building = Building::new(building_type);
        building.body_mut().frame.x = camera_vieport.x + frame.x;
        building.body_mut().frame.y = camera_vieport.y + frame.y;
        let update = WorldStateUpdate::AddEntity(Box::new(building));
        vec![update]
    }
}

impl MapEditor {
    pub fn ui(&self) -> View {
        padding(
            Spacing::LG,
            zstack!(
                Spacing::LG,
                Color::BLACK,
                if let Some(item) = self.item_being_placed {
                    self.placement_ui(&item)
                } else {
                    self.regular_ui()
                }
            )
        )
    }

    fn placement_ui(&self, item: &MapEditorItemBeingPlaced) -> View {
        vstack!(
            Spacing::MD,
            text!(TextStyle::LargeTitle, "Map Editor".to_string()),
            text!(TextStyle::Regular, "Press SPACE to place\nPress ESC to go back".to_string()),
            with_fixed_position(
                Vector2d::new(item.frame.x, item.frame.y),
                zstack!(Spacing::ZERO, Color::RED, spacing!(Spacing::Custom(item.frame.w)))
            )   
        )
    }

    fn regular_ui(&self) -> View {
        vstack!(
            Spacing::LG, 
            text!(TextStyle::Title, "MapEditor".to_string()),
            text!(TextStyle::Regular, "Press SPACE to select something".to_string()),
            View::VGrid {                        
                spacing: GridSpacing::SM(),
                columns: self.columns,
                children: self.stock.iter().enumerate().map(|(index, item)| {
                    item.ui(self.sprite_sheet, index, self.selected_index)
                }).collect()
            }
        )
    }
}

impl MapEditorItem {
    pub fn ui(&self, sprite_sheet: u32, index: usize, selected_index: usize) -> View {
        if index == selected_index {
            zstack!(
                Spacing::XS, 
                Color::YELLOW,
                texture!(
                    sprite_sheet, 
                    self.item.texture_source_rect(), 
                    Vector2d::new(
                        TILE_SIZE_X1_5 - 2.0 * Spacing::XS.unscaled_value(), 
                        TILE_SIZE_X1_5 - 2.0 * Spacing::XS.unscaled_value()
                    )
                )
            )
        } else {
            texture!(
                sprite_sheet, 
                self.item.texture_source_rect(), 
                Vector2d::new(TILE_SIZE_X1_5, TILE_SIZE_X1_5)
            )
        }
    }
}