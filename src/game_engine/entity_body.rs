use std::fmt::Debug;

use raylib::math::{Rectangle, Vector2};

use crate::{constants::TILE_SIZE, utils::geometry_utils::{Insets, IntRect}};

use super::entity::EntityProps;

pub trait EmbodiedEntity: Debug {
    fn id(&self) -> u32;    
    fn parent_id(&self) -> u32;

    fn body(&self) -> &EntityBody;
    fn body_mut(&mut self) -> &mut EntityBody;
    fn collision_frame(&self) -> Rectangle;
    
    fn center_in(&mut self, value: &Rectangle);
    fn place_at(&mut self, x: f32, y: f32);
    fn snap_to_nearest_tile(&mut self);
    fn props(&self) -> EntityProps;
}

#[derive(Debug)]
pub struct EntityBody {
    pub id: u32,
    pub parent_id: u32,
    pub frame: IntRect,  
    pub offset: Vector2,
    pub collision_insets: Insets,
    pub direction: Vector2,
    pub current_speed: f32,
    pub base_speed: f32,
    pub hp: f32,
    pub dp: f32,
    pub creation_time: f32,
    pub requires_collision_detection: bool,
    pub is_rigid: bool,
    pub z_index: i32,
    pub is_ally: bool,
    pub is_bullet: bool,
    pub lifespan: f32,
}

impl EntityBody {            
    pub fn center_in(&mut self, other: &IntRect) {
        let (x, y) = other.center();
        self.center_at(x, y);
    }
    
    pub fn center_at(&mut self, x: i32, y: i32) {
        self.frame.center_at(x, y)
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.frame.resize(width, height)
    }
            
    pub fn reset_speed(&mut self) {
        self.scale_speed(1.0);
    }
            
    pub fn scale_speed(&mut self, scalar: f32) {
        self.current_speed = self.base_speed * scalar;
    }

    pub fn collision_frame(&self) -> IntRect {
        self.collision_insets.apply_to_rect(&self.frame)
    }

    pub fn snap_to_nearest_tile(&mut self) {
        if self.direction.x > 0.0 {
            self.frame.x = (self.frame.x as f32 / TILE_SIZE as f32).floor() * TILE_SIZE;
        } else {
            self.frame.x = (self.frame.x as f32 / TILE_SIZE as f32).ceil() * TILE_SIZE;
        }
        if self.direction.y > 0.0 {
            self.frame.y = (self.frame.y as f32 / TILE_SIZE as f32).floor() * TILE_SIZE;
        } else {
            self.frame.y = (self.frame.y as f32 / TILE_SIZE as f32).ceil() * TILE_SIZE;
        }
    }

    pub fn props(&self) -> EntityProps {
        EntityProps { 
            direction: self.direction, 
            frame: self.frame, 
            speed: self.current_speed 
        }
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::{Rectangle, Vector2};

    use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, game_engine::entity_factory::get_next_entity_id, utils::geometry_utils::Insets};

    use super::EntityBody;

    impl EntityBody {
        pub fn test() -> Self {
            EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame: Rectangle::new(0.0, 0.0, 50.0, 50.0),
                collision_insets: Insets::zero(),
                direction: Vector2::new(0.0, 0.0),
                current_speed: 1.0,
                base_speed: 1.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: true,
                z_index: 0,
                is_ally: false,
                is_bullet: false,
                lifespan: INFINITE_LIFESPAN,
            }
        }
    }
}