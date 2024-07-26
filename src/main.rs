mod constants;
mod entities;
mod game;
mod species;
mod sprites;
mod utils;

use std::{collections::HashMap, path::Path};

use constants::{ASSETS_PATH, FONTS_PATH, FONT_DEBUG, SPECIES_PATH};
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

    let mut square_pos = Vector2::new(100.0, 100.0);
    let square_size = 50.0;
    let mut dragging = false;

    while !rl.window_should_close() {
        let frame_time = (rl.get_frame_time() * 1000.0) as u64;
        game.update(frame_time);        

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_text(format!("{:#?}", game).as_str(), 10, 10, 18, Color::WHITE);

        let mouse_pos = d.get_mouse_position();

        if d.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
            if mouse_pos.x >= square_pos.x && mouse_pos.x <= square_pos.x + square_size
                && mouse_pos.y >= square_pos.y && mouse_pos.y <= square_pos.y + square_size
            {
                dragging = true;
            }
        }

        if d.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
            dragging = false;
        }

        if dragging {
            square_pos = Vector2::new(mouse_pos.x - square_size / 2.0, mouse_pos.y - square_size / 2.0);
        }

        d.draw_rectangle(square_pos.x as i32, square_pos.y as i32, square_size as i32, square_size as i32, Color::GREEN);


        for item in game.render() {            
            if let Some(texture) = textures.get(&item.sprite_path) {
                d.draw_texture_ex(
                    texture,
                    Vector2::new(item.frame.x, item.frame.y),
                    item.z_rotation,
                    item.frame.width / texture.width as f32, 
                    Color::WHITE 
                );
            }
        }
    }
}
