mod constants;
mod game_behaviors;
mod game_engine;
mod features;
mod species;
mod sprites;
mod utils;

use std::collections::HashMap;

use constants::{ASSETS_PATH, DEBUG_ENABLED, FPS, SPECIES_PATH};
use features::mouse_handler::MouseHandler;
use game_engine::{entity::Entity, entity_factory::EntityFactory, game::Game, game_update::GameEngine};
use raylib::prelude::*;
use utils::file_utils::list_files;

struct StateStuff {
    textures: HashMap<String, Texture2D>
}

impl StateStuff {
    fn new() -> Self {
        Self {
            textures: HashMap::new()
        }
    }

    fn load_textures(&mut self, all_assets: &Vec<String>, rl: &mut RaylibHandle, thread: &RaylibThread) {    
        for asset in all_assets {
            let texture = rl.load_texture(&thread, asset).unwrap();
            self.textures.insert(asset.clone(), texture);
        }
    } 
}

fn main() {
    let mut state = StateStuff::new();
    let mut mouse_handler = MouseHandler::new();
    let mut frames_counter = 0;

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello World")
        .build();

    rl.set_target_fps(FPS);

    let all_assets = list_files(ASSETS_PATH, "png");
    let all_species = list_files(SPECIES_PATH, "json");
    state.load_textures(&all_assets, &mut rl, &thread);

    let engine = GameEngine::new();
    let mut game = Game::new(
        EntityFactory::new(all_species, all_assets),
        Rectangle::new(0.0, 0.0, 800.0, 600.0)
    );
    game.setup();

    while !rl.window_should_close() {
        engine.update(&mut game, rl.get_frame_time());
        
        mouse_handler.handle_mouse_event(
            &mut game, 
            rl.get_mouse_position(),
            rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
            rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        );

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for item in game.entities.values() {
            draw_item(&mut d, &item, &mouse_handler, &state);
        }

        frames_counter += 1;

        if DEBUG_ENABLED {
            println!("#{} {:#?}", frames_counter, game);
            // d.draw_text(format!("{:#?}", game).as_str(), 10, 10, 18, Color::WHITE);
        }
    }
}

fn draw_item(
    d: &mut RaylibDrawHandle, 
    item: &Entity,
    mouse: &MouseHandler,
    state: &StateStuff
) {
    let sprite_path = item.current_sprite.current_frame();
    
    if let Some(texture) = state.textures.get(sprite_path) {
        let is_being_dragged = mouse.dragging_id == Some(item.id);
        let dx = if is_being_dragged { mouse.drag_offset.x } else { 0.0 };
        let dy = if is_being_dragged { mouse.drag_offset.y } else { 0.0 };

        d.draw_texture_ex(
            texture,
            Vector2::new(item.frame.x + dx, item.frame.y + dy),
            0.0,
            item.frame.width / texture.width as f32, 
            Color::WHITE 
        );
    }
}