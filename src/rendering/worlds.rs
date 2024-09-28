use raylib::prelude::*;

use crate::game_engine::{engine::GameEngine, world::World};

use super::{death_screen::render_death_screen, entities::render_entities, loading_screen::render_loading_screen, menus::render_menu, tiles::render_tiles};

pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, world: &World, engine: &GameEngine) {
    let fps = rl.get_fps();
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    
    if !engine.loading_screen.is_in_progress() || engine.loading_screen.progress() > 0.4 {
        render_tiles(&mut d, world, engine);
        render_entities(&mut d, world, engine);
    }
    render_menu(&mut d, engine);
    render_loading_screen(&mut d, &engine.loading_screen);
    render_death_screen(&mut d, engine);
    
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