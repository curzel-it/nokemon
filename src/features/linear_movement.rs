use crate::{constants::{BASE_ENTITY_SPEED, TILE_SIZE}, game_engine::{entity::Entity, entity_body::EntityBody, world::World}, utils::{rect::Rect, vector::Vector2d}};

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
    body.direction
        .scaled(body.current_speed)
        .scaled(time_since_last_update)
        .scaled(BASE_ENTITY_SPEED) + body.offset
}

fn would_exit_bounds(frame: &Rect, direction: &Vector2d, bounds: &Rect) -> bool {
    if direction.x > 0.0 && (frame.x + frame.w) >= (bounds.x + bounds.w) {
        return true
    }
    if direction.x < 0.0 && frame.x <= bounds.x {
        return true
    }
    if direction.y > 0.0 && (frame.y + frame.h) >= (bounds.y + bounds.h) {
        return true
    }
    if direction.y < 0.0 && frame.y <= bounds.y {
        return true
    }
    false
}

fn would_collide(frame: &Rect, direction: &Vector2d, hitmap: &Vec<Vec<bool>>) -> bool {
    if direction.x > 0.0 {
        return hitmap[frame.y as usize][frame.x as usize + 1]
    }
    if direction.x < 0.0 {
        return hitmap[frame.y as usize][frame.x as usize - 1]
    }
    if direction.y > 0.0 {
        return hitmap[frame.y as usize + 1][frame.x as usize]
    }
    if direction.y < 0.0 {
        return hitmap[frame.y as usize - 1][frame.x as usize]
    }
    false
}