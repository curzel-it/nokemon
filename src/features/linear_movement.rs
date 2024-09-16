use crate::{constants::{BASE_ENTITY_SPEED, HERO_ENTITY_ID, TILE_SIZE}, game_engine::{entity::Entity, world::World}, utils::{directions::Direction, rect::Rect, vector::Vector2d}};

use super::hitmap::Hitmap;

impl Entity {
    pub fn move_linearly(&mut self, world: &World, time_since_last_update: f32) { 
        let frame = self.frame;
        self.latest_movement = (0, 0);

        if self.current_speed == 0.0 || matches!(self.direction, Direction::Unknown) {
            return
        }
        if would_exit_bounds(&frame, &self.direction, &world.bounds) {
            return
        }
        if self.is_rigid {
            if would_collide(&frame, &self.direction, &world.hitmap) {
                return
            }
            if !can_step_over_hero(self) && would_collide_with_hero(&frame, &self.direction, world) {
                return
            }
        }
        
        let updated_offset = updated_offset(&self.offset, &self.direction, self.current_speed, time_since_last_update);    
        let tiles_x_f = updated_offset.x / TILE_SIZE;
        let tiles_y_f = updated_offset.y / TILE_SIZE;
        let tiles_x = if updated_offset.x > 0.0 { tiles_x_f.floor() } else { tiles_x_f.ceil() };
        let tiles_y = if updated_offset.y > 0.0 { tiles_y_f.floor() } else { tiles_y_f.ceil() };
        let tiles_x_i = tiles_x as i32;
        let tiles_y_i = tiles_y as i32;
        
        self.frame = frame.offset(tiles_x_i, tiles_y_i);
        self.latest_movement = (tiles_x_i, tiles_y_i);

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

fn can_step_over_hero(entity: &Entity) -> bool {
    entity.id == HERO_ENTITY_ID || entity.melee_attacks_hero
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

pub fn would_collide(frame: &Rect, direction: &Direction, hitmap: &Hitmap) -> bool {
    let (col_offset, row_offset) = direction.as_col_row_offset();
    let base_y = frame.y + frame.h - 1;
    let base_x = frame.x;
    hitmap[(base_y + row_offset).max(0) as usize][(base_x + col_offset).max(0) as usize]
}

pub fn would_collide_with_hero(frame: &Rect, direction: &Direction, world: &World) -> bool {
    let (col_offset, row_offset) = direction.as_col_row_offset();
    let y = frame.y + frame.h - 1 + row_offset;
    let x = frame.x + col_offset;
    let hero = world.cached_hero_props.hittable_frame;
    hero.x == x && hero.y == y 
}