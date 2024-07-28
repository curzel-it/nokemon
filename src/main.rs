mod constants;
mod entities;
mod entity_capabilities;
mod features;
mod game;
mod game_capabilities;
mod species;
mod sprites;
mod utils;

use std::collections::HashMap;

use constants::{ASSETS_PATH, DEBUG_ENABLED, FPS, SPECIES_PATH};
use entities::factory::EntityFactory;
use features::item_finder::find_item;
use game::{game::Game, rendered_item::RenderedItem};
use raylib::prelude::*;
use utils::file_utils::list_files;

struct StateStuff {
    textures: HashMap<String, Texture2D>,
    dragging_id: Option<u32>,
    mouse_down: Vector2,
    drag_offset: Vector2,
    reset_dragging_id: bool
}

impl StateStuff {
    fn new() -> Self {
        Self {
            textures: HashMap::new(),
            dragging_id: None,
            mouse_down: Vector2::zero(),
            drag_offset: Vector2::zero(),
            reset_dragging_id: false
        }
    }

    fn load_textures(&mut self, all_assets: &Vec<String>, rl: &mut RaylibHandle, thread: &RaylibThread) {    
        for asset in all_assets {
            let texture = rl.load_texture(&thread, asset).unwrap();
            self.textures.insert(asset.clone(), texture);
        }
    } 

    fn handle_mouse(
        &mut self, 
        game: &mut Game,
        items: &Vec<RenderedItem>,
        position: Vector2, 
        is_pressed: bool, 
        is_released: bool        
    ) {
        self.drag_offset = Vector2::new(
            position.x - self.mouse_down.x, 
            position.y - self.mouse_down.y
        );
        if self.reset_dragging_id {
            self.reset_dragging_id = false;
            self.dragging_id = None;
        }

        if is_pressed {
            if self.dragging_id.is_none() {
                let pointed_item = find_item(position, items).map(|e| e.id);
                self.dragging_id = pointed_item;
                self.mouse_down = position;
            }
        }

        if is_released {
            if let Some(id) = self.dragging_id {
                game.move_entity_by(id, self.drag_offset);
            }
            self.reset_dragging_id = true;
        }
    }
}

fn main() {
    let mut state = StateStuff::new();

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello World")
        .build();

    rl.set_target_fps(FPS);

    let all_assets = list_files(ASSETS_PATH, "png");
    let all_species = list_files(SPECIES_PATH, "json");
    state.load_textures(&all_assets, &mut rl, &thread);

    let mut game = Game::new(
        EntityFactory::new(all_species, all_assets),
        Rectangle::new(0.0, 0.0, 800.0, 600.0)
    );

    while !rl.window_should_close() {
        game.update(rl.get_frame_time());
        let items = game.render();

        state.handle_mouse(
            &mut game, 
            &items,
            rl.get_mouse_position(),
            rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
            rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        );

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for item in items {
            draw_item(&mut d, &item, &state);
        }

        if DEBUG_ENABLED {
            d.draw_text(format!("{:#?}", game).as_str(), 10, 10, 18, Color::WHITE);
        }
    }
}

fn draw_item(
    d: &mut RaylibDrawHandle, 
    item: &RenderedItem,
    state: &StateStuff
) {
    if let Some(texture) = state.textures.get(&item.sprite_path) {
        let is_being_dragged = state.dragging_id == Some(item.id);
        let dx = if is_being_dragged { state.drag_offset.x } else { 0.0 };
        let dy = if is_being_dragged { state.drag_offset.y } else { 0.0 };

        d.draw_texture_ex(
            texture,
            Vector2::new(item.frame.x + dx, item.frame.y + dy),
            item.z_rotation,
            item.frame.width / texture.width as f32, 
            Color::WHITE 
        );
    }
}