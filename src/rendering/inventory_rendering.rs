use raylib::prelude::*;

use crate::{constants::{ASSETS_PATH, TILE_SIZE}, features::inventory::{InventoryItem, InventoryItemBeingPlaced}, game_engine::game_engine::GameEngine, ui::ui::{TextStyle, UiView}, utils::geometry_utils::Scalable};

pub fn render_inventory(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    if engine.inventory.is_open {
        let sprite_path = engine.inventory.sprite_sheet_path();

        for (index, entry) in engine.inventory.stock.iter().enumerate() {
            draw_item(d, sprite_path, entry, index == engine.inventory.selected_index, index, engine);
        }
        if !engine.inventory.is_placing_item {
            // d.draw_text("Use arrows, then space bar", 20, 20, 18, Color::BLACK);
            UiView::Box { 
                padding: 20.0,
                background_color: Color::BLACK,
                children: vec![
                    UiView::Column { 
                        spacing: 20.0,
                        children: vec![
                            UiView::Text { style: TextStyle::Bold, text: "Hello Bold".to_owned() },
                            UiView::Text { style: TextStyle::Regular, text: "Hello Regular".to_owned() },
                            UiView::Texture { 
                                key: format!("{}/inventory.png", ASSETS_PATH), 
                                source_rect: Rectangle::new(TILE_SIZE, 0.0, TILE_SIZE, TILE_SIZE), 
                                size: Vector2::new(TILE_SIZE, TILE_SIZE), 
                            },
                            UiView::Texture { 
                                key: format!("{}/inventory.png", ASSETS_PATH), 
                                source_rect: Rectangle::new(TILE_SIZE, 0.0, TILE_SIZE, TILE_SIZE), 
                                size: Vector2::new(2.0 * TILE_SIZE, 2.0 * TILE_SIZE), 
                            }
                        ], 
                    }
                ]
            }.render(
                d, 
                &engine.ui_config.as_ref().unwrap(), 
                Vector2::new(20.0, 20.0)
            );
        }
        if let Some(item_being_placed) = engine.inventory.item_being_placed {
            draw_placement_indicator(d, item_being_placed, engine);
        }
    }
}

fn draw_item(
    d: &mut RaylibDrawHandle, 
    sprite_path: &str,
    entry: &InventoryItem,
    is_selected: bool,
    index: usize,
    engine: &GameEngine
) {
    let source_rect = entry.item.texture_source_rect();
    
    if let Some(texture) = engine.ui_config.as_ref().unwrap().get_texture(sprite_path) {
        let dest_rect = Rectangle {
            x: index as f32 * (2.0 + TILE_SIZE) + 100.0, 
            y: engine.camera_viewport.height - TILE_SIZE * 2.0,
            width: TILE_SIZE,
            height: TILE_SIZE,
        }.scaled(engine.ui_config.as_ref().unwrap().rendering_scale);

        d.draw_rectangle(
            dest_rect.x as i32 - 4, 
            dest_rect.y as i32 - 4, 
            dest_rect.width as i32 + 8, 
            dest_rect.height as i32 + 8, 
            item_background_color(is_selected)
        );

        d.draw_texture_pro(
            texture,
            source_rect,
            dest_rect,
            Vector2::zero(), 
            0.0,
            Color::WHITE,
        );

        d.draw_text(format!("{}", entry.stock).as_str(), dest_rect.x as i32, dest_rect.y as i32, 12, Color::ORANGE);
    }
}

fn item_background_color(is_selected: bool) -> Color {
    if is_selected {
        Color::YELLOW
    } else {
        Color::BLACK
    }
}

fn draw_placement_indicator(
    d: &mut RaylibDrawHandle, 
    item: InventoryItemBeingPlaced,
    engine: &GameEngine
) {
    let dest_rect = item.frame.scaled(engine.ui_config.as_ref().unwrap().rendering_scale);

    d.draw_rectangle(
        dest_rect.x as i32, 
        dest_rect.y as i32, 
        dest_rect.width as i32, 
        dest_rect.height as i32, 
        Color::RED
    );
}