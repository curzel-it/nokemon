use raylib::prelude::*;

use crate::{combat::screen::FightScreen, game_engine::engine::GameEngine, ui::components::{render_from, AnchorPoint, RenderingConfig}, utils::vector::Vector2d};

pub fn render_combat_screen(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    if !engine.fight_screen.is_open {
        return
    }
    let ui_config = engine.ui_config.as_ref().unwrap();
    let aspect_ratio = ui_config.canvas_size.x / ui_config.canvas_size.y;
    let aspect_ratio_3_2: f32 = 3.0 / 2.0;

    let offset = if aspect_ratio > aspect_ratio_3_2 {
        ((ui_config.canvas_size.x - aspect_ratio_3_2 * ui_config.canvas_size.y) / 2.0, 0.0)
    } else {
        (0.0, 0.0)
    };

    render_background(d);
    render_enemy_life(d, ui_config, offset, &engine.fight_screen);
    render_enemy_avatar(d, ui_config, offset, &engine.fight_screen);
    render_battle_info(d, ui_config, &engine.fight_screen);
    render_player_life(d, ui_config, offset, &engine.fight_screen);
    player_enemy_avatar(d, ui_config, offset, &engine.fight_screen);
    render_player_options(d, ui_config, offset, &engine.fight_screen);
}

fn render_background(d: &mut RaylibDrawHandle) {
    d.draw_rectangle(
        0, 
        0, 
        d.get_screen_width(), 
        d.get_screen_height(), 
        Color::BLACK
    );
}

fn render_battle_info(d: &mut RaylibDrawHandle, ui_config: &RenderingConfig, fight_screen: &FightScreen) {
    render_from(
        AnchorPoint::Center,
        &fight_screen.battle_info_ui(),
        d, 
        ui_config, 
        &Vector2d::new(
            ui_config.canvas_size.x / 2.0,
            ui_config.canvas_size.y / 2.0
        )
    );
}

fn render_enemy_life(d: &mut RaylibDrawHandle, ui_config: &RenderingConfig, offset: (f32, f32), fight_screen: &FightScreen) {
    render_from(
        AnchorPoint::TopLeft,
        &fight_screen.enemy_life_ui(),
        d, 
        ui_config, 
        &Vector2d::new(
            offset.0,
            offset.1
        )
    );
}

fn render_enemy_avatar(d: &mut RaylibDrawHandle, ui_config: &RenderingConfig, offset: (f32, f32), fight_screen: &FightScreen) {
    render_from(
        AnchorPoint::TopRight,
        &fight_screen.enemy_avatar_ui(),
        d, 
        ui_config, 
        &Vector2d::new(
            ui_config.canvas_size.x - offset.0,
            offset.1
        )
    );
}

fn player_enemy_avatar(d: &mut RaylibDrawHandle, ui_config: &RenderingConfig, offset: (f32, f32), fight_screen: &FightScreen) {
    render_from(
        AnchorPoint::BottomLeft,
        &fight_screen.player_avatar_ui(),
        d, 
        ui_config, 
        &Vector2d::new(
            offset.0, 
            ui_config.canvas_size.y - offset.1
        )
    );
}

fn render_player_life(d: &mut RaylibDrawHandle, ui_config: &RenderingConfig, offset: (f32, f32), fight_screen: &FightScreen) {
    render_from(
        AnchorPoint::BottomLeft,
        &fight_screen.player_life_ui(),
        d, 
        ui_config, 
        &Vector2d::new(
            offset.0, 
            ui_config.canvas_size.y - offset.1
        )
    );
}

fn render_player_options(d: &mut RaylibDrawHandle, ui_config: &RenderingConfig, offset: (f32, f32), fight_screen: &FightScreen) {
    render_from(
        AnchorPoint::BottomRight,
        &fight_screen.player_options_ui(),
        d, 
        ui_config, 
        &Vector2d::new(
            ui_config.canvas_size.x - offset.0,
            ui_config.canvas_size.y - offset.1
        )
    );
}