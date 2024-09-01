use crate::{constants::{BASE_ENTITY_SPEED, TILE_SIZE}, game_engine::{entity::Entity, entity_body::EntityBody, world::World}, utils::{directions::Direction, rect::Rect, vector::Vector2d}};

use super::hitmap::Hitmap;

pub fn move_linearly(entity: &mut dyn Entity, world: &World, time_since_last_update: f32) { 
    let body = entity.body();

    if would_exit_bounds(&body.frame, &body.direction, &world.bounds) {
        return
    }
    if would_collide(&body.frame, &body.direction, &world.hitmap) {
        return
    }
    
    let updated_offset = updated_offset(body, time_since_last_update);    
    let tiles_x_f = updated_offset.x / TILE_SIZE;
    let tiles_y_f = updated_offset.y / TILE_SIZE;
    let tiles_x = if updated_offset.x > 0.0 { tiles_x_f.floor() } else { tiles_x_f.ceil() };
    let tiles_y = if updated_offset.y > 0.0 { tiles_y_f.floor() } else { tiles_y_f.ceil() };

    entity.body_mut().frame = entity.body().frame.offset(
        tiles_x as i32, 
        tiles_y as i32
    );

    if tiles_x != 0.0 || tiles_y != 0.0 {
        entity.body_mut().offset = Vector2d::zero();
    } else {
        entity.body_mut().offset = Vector2d::new(
            updated_offset.x - tiles_x * TILE_SIZE,
            updated_offset.y - tiles_y * TILE_SIZE
        );
    }
}

fn updated_offset(body: &EntityBody, time_since_last_update: f32) -> Vector2d {
    body.direction.as_vector()
        .scaled(body.current_speed)
        .scaled(time_since_last_update)
        .scaled(BASE_ENTITY_SPEED) + body.offset
}

fn would_exit_bounds(frame: &Rect, direction: &Direction, bounds: &Rect) -> bool {
    match direction {
        Direction::Up => frame.y <= bounds.y,
        Direction::Right => (frame.x + frame.w) >= (bounds.x + bounds.w),
        Direction::Down => (frame.y + frame.h) >= (bounds.y + bounds.h),
        Direction::Left => frame.x <= bounds.x,
        Direction::Unknown => false
    }
}

fn would_collide(frame: &Rect, direction: &Direction, hitmap: &Hitmap) -> bool {
    let (col_offset, row_offset) = direction.as_col_row_offset();
    let base_y = (frame.y + frame.h - 1) as i32;
    let base_x = frame.x as i32;
    return hitmap[(base_y + row_offset).max(0) as usize][(base_x + col_offset).max(0) as usize]
}