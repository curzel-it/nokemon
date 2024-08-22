use raylib::{color::Color, math::{Rectangle, Vector2}};

use crate::{constants::{ASSETS_PATH, INFINITE_STOCK, TILE_SIZE, TILE_SIZE_X1_5}, entities::building::{Building, BuildingType}, game_engine::{entity_body::EmbodiedEntity, keyboard_events_provider::KeyboardState, state_updates::WorldStateUpdate}, levels::constants::LEVEL_ID_HOUSE_INTERIOR, maps::{biome_tiles::Biome, constructions_tiles::Construction}, text, texture, ui::ui::{padding, GridSpacing, Spacing, TextStyle, View}, vstack, zstack};

#[derive(Debug)]
pub struct Inventory {
    pub is_open: bool,
    pub is_placing_item: bool,
    pub stock: Vec<InventoryItem>,
    pub selected_index: usize,
    pub item_being_placed: Option<InventoryItemBeingPlaced>,
    sprite_sheet_path: String,
    columns: usize
}

#[derive(Debug)]
pub struct InventoryItem {
    pub item: Stockable,
    pub stock: i32
}

#[derive(Debug, Clone, Copy)]
pub struct InventoryItemBeingPlaced {
    pub item: Stockable,
    pub frame: Rectangle
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            is_open: false,
            is_placing_item: false,
            stock: vec![],
            selected_index: 0,
            item_being_placed: None,
            sprite_sheet_path: format!("{}/inventory.png", ASSETS_PATH),
            columns: 5
        }
    }

    pub fn update(&mut self, camera_vieport: &Rectangle, keyboard_state: &KeyboardState) -> Vec<WorldStateUpdate> {
        if !self.is_open && keyboard_state.has_inventory_been_pressed {
            self.is_open = true;
        }
        if !self.is_open {
            return vec![];
        }
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
            if keyboard_state.has_confirmation_been_pressed {
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
            if keyboard_state.has_confirmation_been_pressed {
                self.item_being_placed = Some(
                    InventoryItemBeingPlaced {
                        item: self.stock[self.selected_index].item,
                        frame: Rectangle::new(
                            (camera_vieport.x / TILE_SIZE).ceil() * TILE_SIZE - camera_vieport.x,
                            (camera_vieport.y / TILE_SIZE).ceil() * TILE_SIZE - camera_vieport.y,
                            TILE_SIZE, 
                            TILE_SIZE
                        )
                    }
                );
                self.is_placing_item = true;
            }
            if keyboard_state.has_back_been_pressed {
                self.is_open = false;
            }
        }
        vec![]
    }

    fn place(&self, camera_vieport: &Rectangle, item: Stockable) -> Vec<WorldStateUpdate> {
        let frame = self.item_being_placed.unwrap().frame;
        let row = ((camera_vieport.y + frame.y) / TILE_SIZE) as usize;
        let col = ((camera_vieport.x + frame.x) / TILE_SIZE) as usize;

        match item {
           Stockable::BiomeTile(biome) => vec![WorldStateUpdate::BiomeTileChange(row, col, biome)],
           Stockable::ConstructionTile(construction) => vec![WorldStateUpdate::ConstructionTileChange(row, col, construction)],
           Stockable::Building(building_type) => self.place_building(camera_vieport, building_type),
        }
    }

    pub fn set_creative_mode(&mut self, is_enabled: bool) {        
        if is_enabled {
            self.stock = vec![];
            Stockable::all_possible_items().into_iter().for_each(|item| {
                self.stock.push(InventoryItem { item, stock: INFINITE_STOCK });
            });
            println!("Loaded up for creative mode: {:#?}", self.stock);
        }
    }

    fn place_building(&self, camera_vieport: &Rectangle, building_type: BuildingType) -> Vec<WorldStateUpdate> {
        let frame = self.item_being_placed.unwrap().frame;
        let mut building = Building::new(building_type, LEVEL_ID_HOUSE_INTERIOR);
        building.body_mut().frame.x = camera_vieport.x + frame.x;
        building.body_mut().frame.y = camera_vieport.y + frame.y;
        let update = WorldStateUpdate::AddEntity(Box::new(building));
        vec![update]
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Stockable {
    BiomeTile(Biome),
    ConstructionTile(Construction),    
    Building(BuildingType),    
}

impl Stockable {
    pub fn all_possible_items() -> Vec<Stockable> {
        vec![
            Stockable::BiomeTile(Biome::Water),
            Stockable::BiomeTile(Biome::Desert),
            Stockable::BiomeTile(Biome::Grass),
            Stockable::BiomeTile(Biome::Rock),
            Stockable::BiomeTile(Biome::Snow),
            Stockable::BiomeTile(Biome::LightWood),
            Stockable::BiomeTile(Biome::DarkWood),
            Stockable::ConstructionTile(Construction::WoodenFence),
            Stockable::Building(BuildingType::House),
        ]
    }

    fn texture_source_rect(&self) -> Rectangle {
        let (row, col) = self.texture_offsets();

        Rectangle {
            x: col as f32 * TILE_SIZE,
            y: row as f32 * TILE_SIZE,
            width: TILE_SIZE, 
            height: TILE_SIZE
        }
    }

    fn texture_offsets(&self) -> (u32, u32) {
         match self {
            Stockable::BiomeTile(biome) => match biome {
                Biome::Nothing => (0, 0),
                Biome::Water => (0, 1),
                Biome::Desert => (0, 2),
                Biome::Grass => (0, 3),
                Biome::Rock => (0, 4),
                Biome::Snow => (0, 5),
                Biome::LightWood => (0, 6),
                Biome::DarkWood => (0, 7),
            },
            Stockable::ConstructionTile(construction) => match construction {
                Construction::Nothing => (1, 0),
                Construction::WoodenFence => (1, 1),
            },
            Stockable::Building(building_type) => match building_type {
                BuildingType::House => (1, 2)
            }
        }
    }
}

impl Inventory {
    pub fn ui(&self) -> View {
        padding(
            Spacing::LG,
            zstack!(
                Spacing::LG,
                Color::BLACK,
                vstack!(
                    Spacing::LG, 
                    text!(TextStyle::Title, "Inventory".to_string()),
                    text!(TextStyle::Regular, "1. Press SPACE to select something\n2. Use arrows to move around\n3. Press SPACE to place it\n4. Press ESC to come back".to_string()),
                    View::VGrid {                        
                        spacing: GridSpacing::SM(),
                        columns: self.columns,
                        children: self.stock.iter().enumerate().map(|(index, item)| {
                            item.ui(self.sprite_sheet_path.clone(), index, self.selected_index)
                        }).collect()
                    }
                )
            )
        )
    }
}

impl InventoryItem {
    pub fn ui(&self, sprite_sheet: String, index: usize, selected_index: usize) -> View {
        if index == selected_index {
            zstack!(
                Spacing::XS, 
                Color::YELLOW,
                texture!(
                    sprite_sheet, 
                    self.item.texture_source_rect(), 
                    Vector2::new(
                        TILE_SIZE_X1_5 - 2.0 * Spacing::XS.unscaled_value(), 
                        TILE_SIZE_X1_5 - 2.0 * Spacing::XS.unscaled_value()
                    )
                )
            )
        } else {
            texture!(
                sprite_sheet, 
                self.item.texture_source_rect(), 
                Vector2::new(TILE_SIZE_X1_5, TILE_SIZE_X1_5)
            )
        }
    }
}