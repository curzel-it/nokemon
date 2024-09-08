use crate::{constants::{BASE_ENTITY_SPEED, TILE_SIZE}, game_engine::{concrete_entity::ConcreteEntity, world::World}, utils::{directions::Direction, rect::Rect, vector::Vector2d}};

use super::hitmap::Hitmap;

impl ConcreteEntity {
    pub fn move_linearly(&mut self, world: &World, time_since_last_update: f32) { 
        if self.current_speed == 0.0 || matches!(self.direction, Direction::Unknown) {
            return
        }
        if would_exit_bounds(&self.frame, &self.direction, &world.bounds) {
            return
        }
        if would_collide(&self.frame, &self.direction, &world.hitmap) {
            return
        }
        
        let updated_offset = updated_offset(&self.offset, &self.direction, self.current_speed, time_since_last_update);    
        let tiles_x_f = updated_offset.x / TILE_SIZE;
        let tiles_y_f = updated_offset.y / TILE_SIZE;
        let tiles_x = if updated_offset.x > 0.0 { tiles_x_f.floor() } else { tiles_x_f.ceil() };
        let tiles_y = if updated_offset.y > 0.0 { tiles_y_f.floor() } else { tiles_y_f.ceil() };

        self.frame = self.frame.offset(tiles_x as i32, tiles_y as i32);

        if tiles_x != 0.0 || tiles_y != 0.0 {
            self.offset = Vector2d::zero();
        } else {
            self.offset = Vector2d::new(
                updated_offset.x - tiles_x * TILE_SIZE,
                updated_offset.y - tiles_y * TILE_SIZE
            );
        }
    }
}

fn updated_offset(offset: &Vector2d, direction: &Direction, speed: f32, time_since_last_update: f32) -> Vector2d {
    direction.as_vector()
        .scaled(speed)
        .scaled(time_since_last_update)
        .scaled(BASE_ENTITY_SPEED) + *offset
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
    let base_y = frame.y + frame.h - 1;
    let base_x = frame.x;
    hitmap[(base_y + row_offset).max(0) as usize][(base_x + col_offset).max(0) as usize]
}