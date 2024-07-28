mod constants;
mod entities;
mod features;
mod game;
mod species;
mod sprites;
mod utils;

use std::collections::HashMap;

use constants::{ASSETS_PATH, DEBUG_ENABLED, SPECIES_PATH};
use entities::factory::EntityFactory;
use game::game::Game;
use raylib::prelude::*;
use utils::file_utils::list_files;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello World")
        .build();

    rl.set_target_fps(60);

    let mut textures: HashMap<String, Texture2D> = HashMap::new();
    let all_assets = list_files(ASSETS_PATH, "png");
    let all_species = list_files(SPECIES_PATH, "json");

    for asset in &all_assets {
        let texture = rl.load_texture(&thread, asset).unwrap();
        textures.insert(asset.clone(), texture);
    }

    let mut game = Game::new(
        EntityFactory::new(all_species, all_assets),
        Rectangle::new(0.0, 0.0, 800.0, 600.0)
    );
    game.add_entity_by_species("ape");
    game.add_entity_by_species("tower");

    let mut dragging_id: Option<u32> = None;
    let mut mouse_down = Vector2::zero();
    let mut drag_offset = Vector2::zero();
    let mut reset_dragging_id = false;

    while !rl.window_should_close() {
        game.update(rl.get_frame_time());
        let items = game.render();

        let mouse_position = rl.get_mouse_position();
        drag_offset = Vector2::new(mouse_position.x - mouse_down.x, mouse_position.y - mouse_down.y);

        if reset_dragging_id {
            reset_dragging_id = false;
            dragging_id = None;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if dragging_id.is_none() {
                for item in &items {
                    if item.frame.check_collision_point_rec(mouse_position) {
                        dragging_id = Some(item.id);
                        mouse_down = mouse_position;
                        break;
                    }
                }
            }
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(id) = dragging_id {
                game.move_entity_by(id, drag_offset);
            }
            reset_dragging_id = true;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for item in items {
            if let Some(texture) = textures.get(&item.sprite_path) {
                let is_being_dragged = dragging_id == Some(item.id);
                let dx = if is_being_dragged { drag_offset.x } else { 0.0 };
                let dy = if is_being_dragged { drag_offset.y } else { 0.0 };

                d.draw_texture_ex(
                    texture,
                    Vector2::new(item.frame.x + dx, item.frame.y + dy),
                    item.z_rotation,
                    item.frame.width / texture.width as f32, 
                    Color::WHITE 
                );
            }
        }

        // if DEBUG_ENABLED {
        //     d.draw_text(format!("{:#?}", game).as_str(), 10, 10, 18, Color::WHITE);
        // }
    }
}
