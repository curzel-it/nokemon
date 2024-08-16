use std::fmt::Debug;

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{SCALE, TILE_SIZE}, utils::geometry_utils::Insets};


pub trait EmbodiedEntity: Debug {
    fn id(&self) -> u32;    
    fn parent_id(&self) -> u32;

    fn body(&self) -> &EntityBody;
    fn body_mut(&mut self) -> &mut EntityBody;
    fn collision_frame(&self) -> Rectangle;
    
    fn center_in(&mut self, value: &Rectangle);
    fn place_at(&mut self, x: f32, y: f32);
    fn snap_to_nearest_tile(&mut self);
}

#[derive(Debug)]
pub struct EntityBody {
    pub id: u32,
    pub parent_id: u32,
    pub frame: Rectangle,  
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
    pub fn center_in(&mut self, value: &Rectangle) {
        let center = Vector2 {
            x: value.x + value.width / 2.0,
            y: value.y + value.height / 2.0,
        };
        self.center_at(&center);
    }
    
    pub fn center_at(&mut self, value: &Vector2) {
        self.frame.x = value.x - self.frame.width / 2.0;
        self.frame.y = value.y - self.frame.height / 2.0;
    }
    
    pub fn resize(&mut self, w: f32, h: f32) {
        self.frame.width = SCALE * w;
        self.frame.height = SCALE * h;
    }
            
    pub fn reset_speed(&mut self) {
        self.scale_speed(1.0);
    }
            
    pub fn scale_speed(&mut self, scalar: f32) {
        self.current_speed = self.base_speed * scalar;
    }

    pub fn collision_frame(&self) -> Rectangle {
        self.collision_insets.apply_to_rect(&self.frame)
    }

    pub fn snap_to_nearest_tile(&mut self) {
        self.frame.x = (self.frame.x / TILE_SIZE).round() * TILE_SIZE;
        self.frame.y = (self.frame.y / TILE_SIZE).round() * TILE_SIZE;
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