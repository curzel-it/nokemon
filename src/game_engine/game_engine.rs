use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, FPS, SPECIES_PATH}, features::entity_locator::EntityLocator, game_behaviors::{check_bullet_collisions::CheckBulletCollisons, cleanup_entities::CleanupEntities, linear_movement::LinearMovement, shooter::Shooter, update_sprites::UpdateSprites}, utils::file_utils::list_files};

use super::{entity_factory::EntityFactory, game::Game, game_behavior::GameBehavior, mouse_events_provider::MouseEventsProvider};
use raylib::prelude::*;

pub struct GameEngine {
    entity_locator: EntityLocator,
    behaviors: Vec<Box<dyn GameBehavior>>,
    pub textures: HashMap<String, Texture2D>,
    pub dragging_id: Option<u32>,
    mouse_down: Vector2,
    pub drag_offset: Vector2,
    reset_dragging_id: bool
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            entity_locator: EntityLocator::new(),
            behaviors: vec![
                Box::new(LinearMovement::new()),
                Box::new(UpdateSprites::new()),
                Box::new(Shooter::new()),
                Box::new(CheckBulletCollisons::new()),
                Box::new(CleanupEntities::new()),
            ],
            textures: HashMap::new(),
            dragging_id: None,
            mouse_down: Vector2::zero(),
            drag_offset: Vector2::zero(),
            reset_dragging_id: false
        }
    }

    pub fn start(&mut self, width: i32, height: i32) -> (Game, RaylibHandle, RaylibThread) {
        let (mut rl, thread) = raylib::init()
            .size(width, height)
            .title("Tower Defense")
            .build();
    
        rl.set_target_fps(FPS);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        let all_species = list_files(SPECIES_PATH, "json");

        self.load_textures(&all_assets, &mut rl, &thread);

        let mut game = Game::new(
            EntityFactory::new(all_species, all_assets),
            Rectangle::new(0.0, 0.0, 800.0, 600.0)
        );
        game.add_entity_by_species("ape");
        game.add_entity_by_species("tower");

        return (game, rl, thread);
    }

    pub fn handle_inputs(&mut self, game: &mut Game, rl: &RaylibHandle) {
        self.handle_mouse_event(game, rl);
    } 

    pub fn update(&self, game: &mut Game, time_since_last_update: f32) {
        let entity_ids: Vec<u32> = game.entities.values().map(|e| e.id).collect();
    
        for behavior in &self.behaviors {
            for id in &entity_ids {
                behavior.update(id, game, time_since_last_update);
            }        
        }
    } 

    fn load_textures(&mut self, all_assets: &Vec<String>, rl: &mut RaylibHandle, thread: &RaylibThread) {    
        for asset in all_assets {
            let texture = rl.load_texture(&thread, asset).unwrap();
            self.textures.insert(asset.clone(), texture);
        }
    } 

    fn handle_mouse_event(&mut self, game: &mut Game, mouse: &dyn MouseEventsProvider) {
        let position = mouse.mouse_position();

        self.drag_offset = Vector2::new(
            position.x - self.mouse_down.x, 
            position.y - self.mouse_down.y
        );
        if self.reset_dragging_id {
            self.reset_dragging_id = false;
            self.dragging_id = None;
        }

        if mouse.is_left_mouse_pressed() {
            if self.dragging_id.is_none() {
                let pointed_item = self.entity_locator.find_by_position(game, &position);
                self.dragging_id = pointed_item;
                self.mouse_down = position;
            }
        }

        if mouse.is_left_mouse_released() {
            if let Some(id) = self.dragging_id {
                game.move_entity_by(id, self.drag_offset);
            }
            self.reset_dragging_id = true;
        }
    }
}