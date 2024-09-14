use raylib::color::Color;

use crate::{constants::{SPRITE_SHEET_INVENTORY, TILE_SIZE}, entities::{known_species::SPECIES_HERO, species::{EntityType, Species, ALL_SPECIES}}, game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate}, lang::localizable::LocalizableText, maps::{biome_tiles::Biome, constructions_tiles::Construction}, prefabs::all::new_building, spacing, text, texture, ui::{components::{with_fixed_position, GridSpacing, Spacing, TextStyle, View}, scaffold::scaffold}, utils::{rect::Rect, vector::Vector2d}, vstack, zstack};

use super::menu::MENU_BORDERS_TEXTURES;

#[derive(Debug)]
pub struct MapEditor {
    stock: Vec<Stockable>,
    state: MapEditorState,
    pub current_world_id: u32,
    columns: usize,
    offset: usize, 
}

#[derive(Debug, Clone)]
enum MapEditorState {
    SelectingItem(usize),
    PlacingItem(usize, Stockable, Rect),
}

impl MapEditor {
    pub fn new() -> Self {
        Self {
            stock: MapEditor::all_possible_items().into_iter().collect(),
            state: MapEditorState::SelectingItem(0),
            current_world_id: 0,
            columns: 8,
            offset: 0, 
        }
    }

    pub fn is_placing_item(&self) -> bool {
        match self.state {
            MapEditorState::PlacingItem(_, _, _) => true,
            MapEditorState::SelectingItem(_) => false,
        }
    }

    pub fn update(&mut self, camera_vieport: &Rect, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        match self.state.clone() {
            MapEditorState::SelectingItem(selected_index) => {
                self.update_item_selection(selected_index, camera_vieport, keyboard)
            },
            MapEditorState::PlacingItem(selected_index, item, frame) => {
                self.update_item_placement(selected_index, item, &frame, camera_vieport, keyboard)
            },
        }
    }

    fn update_item_selection(&mut self, selected_index: usize, camera_vieport: &Rect, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.direction_up.is_pressed {
            if selected_index >= self.columns {
                self.state = MapEditorState::SelectingItem(selected_index - self.columns);            
            } else {
                self.state = MapEditorState::SelectingItem(self.stock.len() - (self.columns + 1 - selected_index));            
            }
        }
        if keyboard.direction_right.is_pressed && selected_index < self.stock.len() - 1 {
            self.state = MapEditorState::SelectingItem(selected_index + 1);
        }
        if keyboard.direction_down.is_pressed {
            if selected_index < self.stock.len() - self.columns {
                self.state = MapEditorState::SelectingItem(selected_index + self.columns);
            } else {
                self.state = MapEditorState::SelectingItem(0);
            }
        } 
        if keyboard.direction_left.is_pressed && selected_index > 0 {
            self.state = MapEditorState::SelectingItem(selected_index - 1);
        }
        if keyboard.has_confirmation_been_pressed {
            self.state = MapEditorState::PlacingItem(
                selected_index, 
                self.stock[selected_index].clone(), 
                self.initial_selection_frame(camera_vieport)
            ) 
        }
        vec![]
    }

    fn initial_selection_frame(&self, camera_vieport: &Rect) -> Rect {
        Rect::new(
            camera_vieport.w / 2,
            camera_vieport.h / 2,
            1,
            1
        )
    }

    fn update_item_placement(
        &mut self, 
        selected_index: usize, 
        item: Stockable, 
        frame: &Rect, 
        camera_vieport: &Rect, 
        keyboard: &KeyboardEventsProvider
    ) -> Vec<WorldStateUpdate> {        
        if keyboard.has_confirmation_been_pressed {
            return self.place_item(item, frame, camera_vieport);
        }
        if keyboard.has_back_been_pressed {
            self.state = MapEditorState::SelectingItem(selected_index);
            return vec![];
        }        
        let updated_frame = self.updated_frame(frame, keyboard);
        self.state = MapEditorState::PlacingItem(selected_index, item.clone(), updated_frame);
        vec![]
    }

    fn place_item(
        &mut self, 
        item: Stockable, 
        frame: &Rect, 
        camera_vieport: &Rect
    ) -> Vec<WorldStateUpdate> {
        let row = (camera_vieport.y + frame.y) as usize;
        let col = (camera_vieport.x + frame.x) as usize;

        match item {
            Stockable::BiomeTile(biome) => vec![WorldStateUpdate::BiomeTileChange(row, col, biome)],
            Stockable::ConstructionTile(construction) => match construction {
                Construction::Nothing => vec![
                    WorldStateUpdate::BiomeTileChange(row, col, Biome::Nothing),
                    WorldStateUpdate::ConstructionTileChange(row, col, Construction::Nothing),
                    WorldStateUpdate::RemoveEntityAtCoordinates(row, col),
                ],
                _ => vec![WorldStateUpdate::ConstructionTileChange(row, col, construction)],
            }
            Stockable::Entity(species) => match species.entity_type {
                EntityType::Building => self.place_building(camera_vieport, frame, &species),
                EntityType::Npc => self.place_convertible(camera_vieport, &frame.offset_y(-1), &species),
                _ => self.place_convertible(camera_vieport, frame, &species)
            }
        }
    }

    fn place_convertible(&self, camera_vieport: &Rect, frame: &Rect, species: &Species) -> Vec<WorldStateUpdate> {
        let mut entity = species.make_entity();
        entity.frame.x = camera_vieport.x + frame.x;
        entity.frame.y = camera_vieport.y + frame.y;
        let update = WorldStateUpdate::AddEntity(Box::new(entity));
        vec![update]
    }

    fn place_building(&self, camera_vieport: &Rect, frame: &Rect, species: &Species) -> Vec<WorldStateUpdate> {
        let x = camera_vieport.x + frame.x;
        let y = camera_vieport.y + frame.y;
        
        new_building(self.current_world_id, x, y, species)
            .into_iter()
            .map(Box::new)
            .map(WorldStateUpdate::AddEntity)
            .collect()
    }

    fn updated_frame(&self, frame: &Rect, keyboard: &KeyboardEventsProvider) -> Rect {
        let mut updated_frame = *frame;

        if keyboard.direction_up.is_pressed {
            updated_frame = updated_frame.offset_y(-1);
        }
        if keyboard.direction_right.is_pressed {
            updated_frame = updated_frame.offset_x(1);
        }
        if keyboard.direction_down.is_pressed {
            updated_frame = updated_frame.offset_y(1);
        }
        if keyboard.direction_left.is_pressed {
            updated_frame = updated_frame.offset_x(-1);
        }     
        updated_frame   
    }
}

#[derive(Debug, Clone)]
enum Stockable {
    BiomeTile(Biome),
    ConstructionTile(Construction),    
    Entity(Species),
}

impl Stockable {
    fn texture_source_rect(&self) -> Rect {
        let (y, x) = match self {
            Stockable::BiomeTile(biome) => match biome {
                Biome::Nothing => (0, 0),
                Biome::Water => (0, 1),
                Biome::Desert => (0, 2),
                Biome::Grass => (0, 3),
                Biome::Rock => (0, 4),
                Biome::Snow => (0, 5),
                Biome::LightWood => (0, 6),
                Biome::DarkWood => (0, 7),
                Biome::DarkRock => (0, 8),
                Biome::Ice => (0, 9),
            },
            Stockable::ConstructionTile(construction) => match construction {
                Construction::Nothing => (6, 1),
                Construction::WoodenFence => (1, 1),
                Construction::DarkRock => (1, 2),
                Construction::LightWall => (1, 3),
                Construction::Counter => (1, 4),
                Construction::Library => (1, 5),
                Construction::TallGrass => (1, 6),
                Construction::Forest => (1, 6),
            },
            Stockable::Entity(species) => species.inventory_texture_offset
        };
        Rect::new(x, y, 1, 1)
    }
}

impl Stockable {
    fn ui(&self, index: usize, selected_index: usize) -> View {
        let selected_size = 1.5 - 2.0 * Spacing::XS.unscaled_value() / TILE_SIZE;

        if index == selected_index {
            zstack!(
                Spacing::XS, 
                Color::YELLOW,
                texture!(
                    SPRITE_SHEET_INVENTORY, 
                    self.texture_source_rect(), 
                    Vector2d::new(selected_size, selected_size)
                )
            )
        } else {
            texture!(
                SPRITE_SHEET_INVENTORY, 
                self.texture_source_rect(), 
                Vector2d::new(1.5, 1.5)
            )
        }
    }
}

impl MapEditor {
    fn all_possible_items() -> Vec<Stockable> {
        let mut all = vec![
            Stockable::BiomeTile(Biome::Water),
            Stockable::BiomeTile(Biome::Desert),
            Stockable::BiomeTile(Biome::Grass),
            Stockable::BiomeTile(Biome::Rock),
            Stockable::BiomeTile(Biome::DarkRock),
            Stockable::BiomeTile(Biome::Snow),
            Stockable::BiomeTile(Biome::LightWood),
            Stockable::BiomeTile(Biome::DarkWood),
            Stockable::BiomeTile(Biome::Ice),
            Stockable::ConstructionTile(Construction::Nothing),
            Stockable::ConstructionTile(Construction::WoodenFence),
            Stockable::ConstructionTile(Construction::DarkRock),
            Stockable::ConstructionTile(Construction::LightWall),
            Stockable::ConstructionTile(Construction::Counter),
            Stockable::ConstructionTile(Construction::Library),
            Stockable::ConstructionTile(Construction::Forest),
        ];
        let mut species: Vec<Stockable> = ALL_SPECIES.iter()
            .filter(|s| s.id != SPECIES_HERO)
            .map(|s| Stockable::Entity(s.clone()))
            .collect();
        all.append(&mut species);
        all
    }
}

impl MapEditor {
    pub fn ui(&self, camera_offset: &Vector2d) -> View {
        scaffold(
            true, 
            self.background_color(),
            Some(MENU_BORDERS_TEXTURES),
            match self.state {
                MapEditorState::SelectingItem(selected_index) => self.regular_ui(selected_index),
                MapEditorState::PlacingItem(_, _, frame) => self.placement_ui(camera_offset, &frame),
            }
        )
    }

    fn background_color(&self) -> Color {
        match self.state {
            MapEditorState::PlacingItem(_, _, _) => Color::BLACK.alpha(0.5),
            MapEditorState::SelectingItem(_) => Color::BLACK
        }
    }

    fn placement_ui(&self, camera_offset: &Vector2d, frame: &Rect) -> View {
        vstack!(
            Spacing::MD,
            text!(TextStyle::Regular, "map_editor.placement".localized()),
            with_fixed_position(
                Vector2d::new(
                    TILE_SIZE * frame.x as f32 - camera_offset.x, 
                    TILE_SIZE * frame.y as f32 - camera_offset.y
                ),
                zstack!(Spacing::Zero, Color::RED, spacing!(Spacing::Custom(TILE_SIZE * frame.w as f32)))
            )   
        )
    }
    
    fn regular_ui(&self, selected_item_index: usize) -> View {
        let mut ui_elements = vec![
            text!(TextStyle::Title, "map_editor.title".localized()),
            text!(TextStyle::Regular, "map_editor.subtitle".localized()),
            View::VGrid {
                spacing: GridSpacing::sm(),
                columns: self.columns,
                children: self.stock.iter().enumerate().map(|(index, item)| {
                    item.ui(index, selected_item_index)
                }).collect()
            },
        ];

        if self.offset > 0 {
            ui_elements.push(text!(TextStyle::Regular, "^".to_string()));
        }

        View::VStack { spacing: Spacing::LG, children: ui_elements }
    }
}