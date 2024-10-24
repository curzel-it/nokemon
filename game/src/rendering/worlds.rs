use game_core::{camera_viewport, camera_viewport_offset, can_render_frame, engine};
use raylib::prelude::*;

use super::{entities::render_entities, tiles::render_tiles, ui::render_layout};

pub fn render_frame(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let engine = engine();
    let world = &engine.world;

    let fps = rl.get_fps();
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    
    if can_render_frame() {
        let camera_viewport = camera_viewport();
        let camera_viewport_offset = camera_viewport_offset();

        render_tiles(
            &mut d, 
            &camera_viewport, 
            &camera_viewport_offset,
            &world.biome_tiles.tiles,
            &world.constructions_tiles.tiles
        );
        render_entities(&mut d, &camera_viewport, &camera_viewport_offset);
    }

    let hud = engine.hud_ui(d.get_screen_width(), d.get_screen_height());
    render_layout(&hud, &mut d);
    
    draw_debug_info(
        &mut d, 
        fps, 
        world.id,
        world.cached_hero_props.hittable_frame.x, 
        world.cached_hero_props.hittable_frame.y
    );
}

fn draw_debug_info(d: &mut RaylibDrawHandle, fps: u32, world_id: u32, hero_x: i32, hero_y: i32) {
    d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::RED);
    d.draw_text(&format!("x {}, y {}", hero_x, hero_y), 10, 40, 20, Color::RED);
    d.draw_text(&format!("World {}", world_id), 10, 70, 20, Color::RED);
}