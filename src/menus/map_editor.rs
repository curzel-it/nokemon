use raylib::color::Color;

use crate::{constants::{TILE_SIZE, WORLD_ID_NONE}, entities::{known_species::SPECIES_TELEPORTER, species::{make_entity_by_species, EntityType, Species, ALL_SPECIES}}, game_engine::{keyboard_events_provider::KeyboardEventsProvider, state_updates::WorldStateUpdate, stockable::Stockable}, lang::localizable::LocalizableText, maps::{biome_tiles::Biome, constructions_tiles::Construction}, prefabs::all::new_building, spacing, text, ui::components::{scaffold_background_backdrop, with_fixed_position, GridSpacing, Spacing, TextStyle, View}, utils::{ids::get_next_id, rect::Rect, vector::Vector2d}, vstack, worlds::utils::{list_worlds_with_none, world_name}, zstack};

const MAX_VISIBLE_WORLDS: usize = 4;

#[derive(Debug)]
pub struct MapEditor {
    pub stock: Vec<Stockable>,
    pub worlds: Vec<u32>,
    pub current_world_id: u32,
    state: MapEditorState,
    columns: usize,
    offset: usize, 
}

#[derive(Debug, Clone)]
enum MapEditorState {
    SelectingItem(usize),
    SelectingWorld(usize),
    PlacingItem(usize, Stockable, Rect),
    PlacingWorld(usize, u32, Rect),
}

impl MapEditor {
    pub fn new() -> Self {
        Self {
            stock: MapEditor::all_possible_items().into_iter().collect(),
            worlds: list_worlds_with_none(),
            current_world_id: WORLD_ID_NONE,
            state: MapEditorState::SelectingItem(0),
            columns: 8,
            offset: 0, 
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

    pub fn update(&mut self, camera_vieport: &Rect, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        match self.state.clone() {
            MapEditorState::SelectingItem(selected_index) => {
                self.update_item_selection(selected_index, camera_vieport, keyboard)
            },
            MapEditorState::SelectingWorld(selected_index) => {
                self.update_world_selection(selected_index, camera_vieport, keyboard)
            },
            MapEditorState::PlacingItem(selected_index, item, frame) => {
                self.update_item_placement(selected_index, item, &frame, camera_vieport, keyboard)
            },
            MapEditorState::PlacingWorld(selected_index, destination_id, frame) => {
                self.update_world_placement(selected_index, destination_id, &frame, camera_vieport, keyboard)
            },
        }
    }

    fn update_item_selection(&mut self, selected_index: usize, camera_vieport: &Rect, keyboard: &KeyboardEventsProvider) -> Vec<WorldStateUpdate> {
        if keyboard.direction_up.is_pressed && selected_index >= self.columns {
            self.state = MapEditorState::SelectingItem(selected_index - self.columns);            
        }
        if keyboard.direction_right.is_pressed && selected_index < self.stock.len() - 1 {
            self.state = MapEditorState::SelectingItem(selected_index + 1);
        }
        if keyboard.direction_down.is_pressed {
            if selected_index < self.stock.len() - self.columns {
                self.state = MapEditorState::SelectingItem(selected_index + self.columns);
            } else {
                self.state = MapEditorState::SelectingWorld(0);
            }
        } 
        if keyboard.direction_left.is_pressed && selected_index > 0 {
            self.state = MapEditorState::SelectingItem(selected_index - 1);
        }
        if keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed {
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

    fn update_world_selection(
        &mut self,
        selected_index: usize,
        camera_vieport: &Rect,
        keyboard: &KeyboardEventsProvider
    ) -> Vec<WorldStateUpdate> {
        let total_worlds = self.worlds.len();
    
        if keyboard.direction_up.is_pressed {
            if selected_index > 0 {
                self.state = MapEditorState::SelectingWorld(selected_index - 1);
            } else {
                self.state = MapEditorState::SelectingItem(self.stock.len() - 1);
                return vec![]; 
            }
        }
    
        if keyboard.direction_down.is_pressed && selected_index < total_worlds - 1 {
            self.state = MapEditorState::SelectingWorld(selected_index + 1);
        }
    
        if selected_index < self.offset {
            self.offset = selected_index;
        } else if selected_index >= self.offset + MAX_VISIBLE_WORLDS {
            self.offset = selected_index - MAX_VISIBLE_WORLDS + 1;
        }
    
        if keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed {
            self.state = MapEditorState::PlacingWorld(
                selected_index,
                self.worlds[selected_index],
                self.initial_selection_frame(camera_vieport)
            );
        }    
        vec![]
    }    

    fn update_world_placement(
        &mut self, 
        selected_index: usize, 
        destination_id: u32,
        frame: &Rect, 
        camera_vieport: &Rect, 
        keyboard: &KeyboardEventsProvider
    ) -> Vec<WorldStateUpdate> {        
        if keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed {
            return self.place_world(destination_id, frame, camera_vieport);
        }
        if keyboard.has_back_been_pressed {
            self.state = MapEditorState::SelectingWorld(selected_index);
            return vec![];
        }        
        let updated_frame = self.updated_frame(frame, keyboard);
        self.state = MapEditorState::PlacingWorld(selected_index, destination_id, updated_frame);
        vec![]
    }

    fn place_world(&self, destination_id: u32, frame: &Rect, camera_vieport: &Rect) -> Vec<WorldStateUpdate> {
        let actual_destination_id = if destination_id == WORLD_ID_NONE { 
            get_next_id()
        } else {
            destination_id
        };
        let mut teleporter = make_entity_by_species(SPECIES_TELEPORTER);
        teleporter.destination = actual_destination_id;
        teleporter.frame.x = camera_vieport.x + frame.x;
        teleporter.frame.y = camera_vieport.y + frame.y;
        let update = WorldStateUpdate::AddEntity(teleporter);
        vec![update]
    }

    fn update_item_placement(
        &mut self, 
        selected_index: usize, 
        item: Stockable, 
        frame: &Rect, 
        camera_vieport: &Rect, 
        keyboard: &KeyboardEventsProvider
    ) -> Vec<WorldStateUpdate> {        
        if keyboard.has_confirmation_been_pressed || keyboard.has_menu_been_pressed {
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
        let update = WorldStateUpdate::AddEntity(entity);
        vec![update]
    }

    fn place_building(&self, camera_vieport: &Rect, frame: &Rect, species: &Species) -> Vec<WorldStateUpdate> {
        let x = camera_vieport.x + frame.x;
        let y = camera_vieport.y + frame.y;
        
        new_building(self.current_world_id, x, y, species)
            .into_iter()
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
            Stockable::ConstructionTile(Construction::Nothing),
            Stockable::ConstructionTile(Construction::WoodenFence),
            Stockable::ConstructionTile(Construction::DarkRock),
            Stockable::ConstructionTile(Construction::LightWall),
        ];
        let mut species: Vec<Stockable> = ALL_SPECIES.iter().map(|s| Stockable::Entity(s.clone())).collect();
        all.append(&mut species);
        all
    }
}

impl MapEditor {
    pub fn ui(&self, camera_offset: &Vector2d) -> View {
        scaffold_background_backdrop(
            true, 
            self.background_color(),
            match self.state {
                MapEditorState::SelectingItem(selected_index) => self.regular_ui(selected_index, 999),
                MapEditorState::SelectingWorld(selected_index) => self.regular_ui(999, selected_index),
                MapEditorState::PlacingItem(_, _, frame) => self.placement_ui(camera_offset, &frame),
                MapEditorState::PlacingWorld(_, _, frame) => self.placement_ui(camera_offset, &frame),
            }
        )
    }

    fn background_color(&self) -> Color {
        match self.state {
            MapEditorState::PlacingItem(_, _, _) => Color::BLACK.alpha(0.3),
            MapEditorState::PlacingWorld(_, _, _) => Color::BLACK.alpha(0.3),
            _ => Color::BLACK
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
    
    fn regular_ui(&self, selected_item_index: usize, selected_world_index: usize) -> View {
        let total_worlds = self.worlds.len();

        let visible_worlds = &self.worlds[self.offset..(self.offset + MAX_VISIBLE_WORLDS).min(total_worlds)];

        let world_views: Vec<View> = visible_worlds.iter().enumerate().map(|(index, &world_id)| {
            let world_index = self.offset + index; 
            let name = world_name(&world_id);
            if world_index == selected_world_index {
                text!(TextStyle::Selected, format!("> {}", name))
            } else {
                text!(TextStyle::Regular, format!("{}", name))
            }
        }).collect();

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

        ui_elements.extend(world_views);

        if self.offset + MAX_VISIBLE_WORLDS < total_worlds {
            ui_elements.push(text!(TextStyle::Regular, "...".to_string()));
        }

        View::VStack { spacing: Spacing::LG, children: ui_elements }
    }
}