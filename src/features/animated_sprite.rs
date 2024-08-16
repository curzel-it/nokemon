use raylib::math::Rectangle;

use crate::{constants::{ANIMATIONS_FPS, ASSETS_PATH}, utils::timed_content_provider::TimedContentProvider};

#[derive(Debug)]
pub struct AnimatedSprite {
    pub sheet_path: String,
    pub row: f32,
    pub frames_provider: TimedContentProvider<f32>,
    pub width: f32,
    pub height: f32
}

impl AnimatedSprite {
    pub fn new(sprite: &str, number_of_frames: u32, width: u32, height: u32) -> Self {
        Self {
            sheet_path: format!("{}/{}.png", ASSETS_PATH, sprite),
            row: 0.0,
            frames_provider: TimedContentProvider::frames_counter(number_of_frames),
            width: width as f32,
            height: height as f32
        }
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.frames_provider.update(time_since_last_update)
    }

    pub fn texture_source_rect(&self) -> Rectangle {
        Rectangle::new(
            self.frames_provider.current_frame() * self.width,
            self.row * self.height,
            self.width,
            self.height
        )
    }
}

impl TimedContentProvider<f32> {
    pub fn frames_counter(n: u32) -> Self {
        let frames = Vec::from_iter((0..n).map(|v| v as f32));
        Self::new(frames, ANIMATIONS_FPS)
    }
}

#[macro_export]
macro_rules! impl_humanoid_sprite_update {
    ($struct_name:ident) => {
        impl $struct_name {
            fn update_sprite(&mut self, time_since_last_update: f32) {
                let direction = $crate::utils::geometry_utils::Direction::from_vector(self.body.direction);
                let is_moving = self.body.current_speed != 0.0;
        
                self.sprite.row = match (direction, is_moving) {
                    ($crate::utils::geometry_utils::Direction::Up, true) => 0.0,
                    (crate::utils::geometry_utils::Direction::Up, false) => 1.0,
                    (crate::utils::geometry_utils::Direction::Right, true) => 2.0,
                    (crate::utils::geometry_utils::Direction::Right, false) => 3.0,
                    (crate::utils::geometry_utils::Direction::Down, true) => 4.0,
                    (crate::utils::geometry_utils::Direction::Down, false) => 5.0,
                    (crate::utils::geometry_utils::Direction::Left, true) => 6.0,
                    (crate::utils::geometry_utils::Direction::Left, false) => 7.0,
                    (crate::utils::geometry_utils::Direction::Unknown, true) => 5.0,
                    (crate::utils::geometry_utils::Direction::Unknown, false) => 5.0
                };
                self.sprite.update(time_since_last_update);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_bullet_sprite_update {
    ($struct_name:ident) => {
        impl $struct_name {
            fn update_sprite(&mut self, time_since_last_update: f32) {
                let direction = $crate::utils::geometry_utils::Direction::from_vector(self.body.direction);
        
                self.sprite.row = match direction {
                    $crate::utils::geometry_utils::Direction::Up => 2.0,
                    crate::utils::geometry_utils::Direction::Right => 0.0,
                    crate::utils::geometry_utils::Direction::Down => 3.0,
                    crate::utils::geometry_utils::Direction::Left => 1.0,
                    crate::utils::geometry_utils::Direction::Unknown => 3.0,
                };
                self.sprite.update(time_since_last_update);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_single_animation_sprite_update {
    ($struct_name:ident) => {
        impl $struct_name {
            fn update_sprite(&mut self, time_since_last_update: f32) {
                self.sprite.update(time_since_last_update);
            }
        }
    };
}
