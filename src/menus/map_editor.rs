use raylib::color::Color;

use crate::{constants::{INFINITE_STOCK, SPRITE_SHEET_INVENTORY, TILE_SIZE, TILE_SIZE_X1_5}, entities::{building::{Building, BuildingType}, teleporter::{self, Teleporter}}, game_engine::{entity_body::EmbodiedEntity, keyboard_events_provider::KeyboardState, state_updates::WorldStateUpdate, world::World}, spacing, text, texture, ui::ui::{padding, with_fixed_position, GridSpacing, Spacing, TextStyle, View}, utils::{rect::Rect, vector::Vector2d}, vstack, worlds::{constants::{WORLD_ID_HOUSE_INTERIOR, WORLD_ID_NONE}, utils::{get_next_world_id, list_worlds, world_name}}, zstack};

use super::inventory::Stockable;

#[derive(Debug)]
pub struct MapEditor {
    pub stock: Vec<MapEditorItem>,
    pub worlds: Vec<u32>,
    state: MapEditorState,
    sprite_sheet: u32,
    columns: usize
}

#[derive(Debug)]
enum MapEditorState {
    SelectingItem(usize),
    SelectingWorld(usize),
    PlacingItem(usize, Stockable, Rect),
    PlacingWorld(usize, u32, Rect),
}

#[derive(Debug)]
pub struct MapEditorItem {
    pub item: Stockable,
    pub stock: i32
}

impl MapEditor {
    pub fn new() -> Self {
        Self {
            stock: Stockable::all_possible_items().into_iter()
                .map(|item| { MapEditorItem { item, stock: INFINITE_STOCK } })
                .collect(),
            worlds: {
                let mut options = list_worlds();
                options.push(WORLD_ID_NONE);
                options
            },
            state: MapEditorState::SelectingItem(0),
            sprite_sheet: SPRITE_SHEET_INVENTORY,
            columns: 5,
        }
    }

    pub fn is_placing_item(&self) -> bool {
        match self.state {
            MapEditorState::PlacingItem(_, _, _) => true,
            MapEditorState::PlacingWorld(_, _, _) => true,
            MapEditorState::SelectingItem(_) => false,
            MapEditorState::SelectingWorld(_) => false,
        }
    }

    pub fn update(&mut self, camera_vieport: &Rect, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        match self.state {
            MapEditorState::SelectingItem(selected_index) => {
                self.update_item_selection(selected_index, camera_vieport, keyboard_state)
            },
            MapEditorState::SelectingWorld(selected_index) => {
                self.update_world_selection(selected_index, camera_vieport, keyboard_state)
            },
            MapEditorState::PlacingItem(selected_index, item, frame) => {
                self.update_item_placement(selected_index, item, &frame, camera_vieport, keyboard_state)
            },
            MapEditorState::PlacingWorld(selected_index, destination_id, frame) => {
                self.update_world_placement(selected_index, destination_id, &frame, camera_vieport, keyboard_state)
            },
        }
    }

    fn update_item_selection(&mut self, selected_index: usize, camera_vieport: &Rect, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if keyboard_state.has_up_been_pressed && selected_index >= self.columns {
            self.state = MapEditorState::SelectingItem(selected_index - self.columns);            
        }
        if keyboard_state.has_right_been_pressed && selected_index < self.stock.len() - 1 {
            self.state = MapEditorState::SelectingItem(selected_index + 1);
        }
        if keyboard_state.has_down_been_pressed {
            if selected_index < self.stock.len() - self.columns {
                self.state = MapEditorState::SelectingItem(selected_index + self.columns);
            } else {
                self.state = MapEditorState::SelectingWorld(0);
            }
        } 
        if keyboard_state.has_left_been_pressed && selected_index > 0 {
            self.state = MapEditorState::SelectingItem(selected_index - 1);
        }
        if keyboard_state.has_confirmation_been_pressed || keyboard_state.has_menu_been_pressed {
            self.state = MapEditorState::PlacingItem(
                selected_index, 
                self.stock[selected_index].item, 
                self.initial_selection_frame(camera_vieport)
            ) 
        }
        vec![]
    }

    fn initial_selection_frame(&self, camera_vieport: &Rect) -> Rect {
        Rect::new(
            (camera_vieport.x / TILE_SIZE).ceil() * TILE_SIZE - camera_vieport.x,
            (camera_vieport.y / TILE_SIZE).ceil() * TILE_SIZE - camera_vieport.y,
            TILE_SIZE, 
            TILE_SIZE
        )
    }

    fn update_world_selection(&mut self, selected_index: usize, camera_vieport: &Rect, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if keyboard_state.has_up_been_pressed {
            if selected_index > 0 {
                self.state = MapEditorState::SelectingWorld(selected_index - 1);            
            } else {
                self.state = MapEditorState::SelectingItem(self.stock.len() - 1);
            }
        }
        if keyboard_state.has_down_been_pressed && selected_index < self.stock.len() - 1 {
            self.state = MapEditorState::SelectingWorld(selected_index + 1);    
        }        
        if keyboard_state.has_confirmation_been_pressed || keyboard_state.has_menu_been_pressed {
            self.state = MapEditorState::PlacingWorld(
                selected_index, 
                self.worlds[selected_index],
                self.initial_selection_frame(camera_vieport)
            ) 
        }
        vec![]
    }

    fn update_world_placement(
        &mut self, 
        selected_index: usize, 
        destination_id: u32,
        frame: &Rect, 
        camera_vieport: &Rect, 
        keyboard_state: &KeyboardState
    ) -> Vec<WorldStateUpdate> {        
        if keyboard_state.has_confirmation_been_pressed || keyboard_state.has_menu_been_pressed {
            return self.place_world(destination_id, frame, camera_vieport);
        }
        if keyboard_state.has_back_been_pressed {
            self.state = MapEditorState::SelectingWorld(selected_index);
            return vec![];
        }        
        let updated_frame = self.updated_frame(frame, keyboard_state);
        self.state = MapEditorState::PlacingWorld(selected_index, destination_id, updated_frame);
        vec![]
    }

    fn place_world(&self, destination_id: u32, frame: &Rect, camera_vieport: &Rect) -> Vec<WorldStateUpdate> {
        let actual_destination_id = if destination_id == WORLD_ID_NONE { 
            get_next_world_id()
        } else {
            destination_id
        };
        let mut teleporter = Teleporter::new(actual_destination_id);
        teleporter.body_mut().frame.x = camera_vieport.x + frame.x;
        teleporter.body_mut().frame.y = camera_vieport.y + frame.y;
        let update = WorldStateUpdate::AddEntity(Box::new(teleporter));
        vec![update]
    }

    fn update_item_placement(
        &mut self, 
        selected_index: usize, 
        item: Stockable, 
        frame: &Rect, 
        camera_vieport: &Rect, 
        keyboard_state: &KeyboardState
    ) -> Vec<WorldStateUpdate> {        
        if keyboard_state.has_confirmation_been_pressed || keyboard_state.has_menu_been_pressed {
            return self.place_item(item, frame, camera_vieport);
        }
        if keyboard_state.has_back_been_pressed {
            self.state = MapEditorState::SelectingItem(selected_index);
            return vec![];
        }        
        let updated_frame = self.updated_frame(frame, keyboard_state);
        self.state = MapEditorState::PlacingItem(selected_index, item, updated_frame);
        vec![]
    }

    fn place_item(
        &mut self, 
        item: Stockable, 
        frame: &Rect, 
        camera_vieport: &Rect
    ) -> Vec<WorldStateUpdate> {
        let row = ((camera_vieport.y + frame.y) / TILE_SIZE) as usize;
        let col = ((camera_vieport.x + frame.x) / TILE_SIZE) as usize;

        match item {
           Stockable::BiomeTile(biome) => vec![WorldStateUpdate::BiomeTileChange(row, col, biome)],
           Stockable::ConstructionTile(construction) => vec![WorldStateUpdate::ConstructionTileChange(row, col, construction)],
           Stockable::Building(building_type) => self.place_building(camera_vieport, frame, building_type),
        }
    }

    fn place_building(&self, camera_vieport: &Rect, frame: &Rect, building_type: BuildingType) -> Vec<WorldStateUpdate> {
        let mut building = Building::new(building_type);
        building.body_mut().frame.x = camera_vieport.x + frame.x;
        building.body_mut().frame.y = camera_vieport.y + frame.y;
        let update = WorldStateUpdate::AddEntity(Box::new(building));
        vec![update]
    }

    fn updated_frame(&self, frame: &Rect, keyboard_state: &KeyboardState) -> Rect {
        let mut updated_frame = frame.clone();

        if keyboard_state.has_up_been_pressed {
            updated_frame = updated_frame.offset_y(-TILE_SIZE);
        }
        if keyboard_state.has_right_been_pressed {
            updated_frame = updated_frame.offset_x(TILE_SIZE);
        }
        if keyboard_state.has_down_been_pressed {
            updated_frame = updated_frame.offset_y(TILE_SIZE);
        }
        if keyboard_state.has_left_been_pressed {
            updated_frame = updated_frame.offset_x(-TILE_SIZE);
        }     
        updated_frame   
    }
}

impl MapEditor {
    pub fn ui(&self) -> View {
        padding(
            Spacing::LG,
            zstack!(
                Spacing::LG,
                Color::BLACK,
                match self.state {
                    MapEditorState::SelectingItem(selected_index) => self.regular_ui(selected_index, 999),
                    MapEditorState::SelectingWorld(selected_index) => self.regular_ui(999, selected_index),
                    MapEditorState::PlacingItem(_, _, frame) => self.placement_ui(&frame),
                    MapEditorState::PlacingWorld(_, _, frame) => self.placement_ui(&frame),
                }
            )
        )
    }

    fn placement_ui(&self, frame: &Rect) -> View {
        vstack!(
            Spacing::MD,
            text!(TextStyle::LargeTitle, "Map Editor".to_string()),
            text!(TextStyle::Regular, "Press SPACE to place\nPress ESC to go back".to_string()),
            with_fixed_position(
                Vector2d::new(frame.x, frame.y),
                zstack!(Spacing::ZERO, Color::RED, spacing!(Spacing::Custom(frame.w)))
            )   
        )
    }

    fn regular_ui(&self, selected_item_index: usize, selected_world_index: usize) -> View {
        vstack!(
            Spacing::LG, 
            text!(TextStyle::Title, "MapEditor".to_string()),
            text!(TextStyle::Regular, "Press SPACE to select something".to_string()),
            View::VGrid {                        
                spacing: GridSpacing::SM(),
                columns: self.columns,
                children: self.stock.iter().enumerate().map(|(index, item)| {
                    item.ui(self.sprite_sheet, index, selected_item_index)
                }).collect()
            },
            View::VStack { 
                spacing: Spacing::SM, 
                children: self.worlds.iter().enumerate().map(|(index, item)| {
                    let name = world_name(item);
                    if index == selected_world_index {
                        text!(TextStyle::Bold, format!("> {}", name))
                    } else {
                        text!(TextStyle::Regular, format!("{}", name))
                    }
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