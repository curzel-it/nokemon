use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{constants::ANIMATIONS_FPS, utils::{rect::Rect, timed_content_provider::TimedContentProvider}};

#[derive(Debug)]
pub struct AnimatedSprite {
    pub sheet_id: u32,
    pub index: f32,
    pub row: f32,
    pub frames_provider: TimedContentProvider<f32>,
    pub width: f32,
    pub height: f32,
    step: u32,
    number_of_frames: u32,
}

impl AnimatedSprite {
    pub fn new(sheet_id: u32, number_of_frames: u32, width: u32, height: u32) -> Self {
        Self {
            sheet_id,
            index: 0.0,
            row: 0.0,
            frames_provider: TimedContentProvider::frames_counter(number_of_frames),
            width: width as f32,
            height: height as f32,
            step: 0,
            number_of_frames,
        }
    }

    pub fn new_stepped(sheet_id: u32, number_of_frames: u32, index: u32, step: u32, width: u32, height: u32) -> Self {
        Self {
            sheet_id,
            index: index as f32,
            row: 0.0,
            frames_provider: TimedContentProvider::stepped_frames_counter(number_of_frames, step),
            width: width as f32,
            height: height as f32,
            step,
            number_of_frames,
        }
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.frames_provider.update(time_since_last_update)
    }

    pub fn texture_source_rect(&self) -> Rect {
        Rect::new(
            (self.index + self.frames_provider.current_frame()) * self.width,
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

    pub fn stepped_frames_counter(n: u32, step: u32) -> Self {
        let frames = Vec::from_iter((0..n).map(|v| step * v).map(|v| v as f32));
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
                    ($crate::utils::geometry_utils::Direction::Up, false) => 1.0,
                    ($crate::utils::geometry_utils::Direction::Right, true) => 2.0,
                    ($crate::utils::geometry_utils::Direction::Right, false) => 3.0,
                    ($crate::utils::geometry_utils::Direction::Down, true) => 4.0,
                    ($crate::utils::geometry_utils::Direction::Down, false) => 5.0,
                    ($crate::utils::geometry_utils::Direction::Left, true) => 6.0,
                    ($crate::utils::geometry_utils::Direction::Left, false) => 7.0,
                    ($crate::utils::geometry_utils::Direction::Unknown, true) => 5.0,
                    ($crate::utils::geometry_utils::Direction::Unknown, false) => 5.0
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
                    $crate::utils::geometry_utils::Direction::Right => 0.0,
                    $crate::utils::geometry_utils::Direction::Down => 3.0,
                    $crate::utils::geometry_utils::Direction::Left => 1.0,
                    $crate::utils::geometry_utils::Direction::Unknown => 3.0,
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

#[derive(Serialize, Deserialize)]
struct AnimatedSpriteData {
    sheet_id: u32,
    index: u32,
    row: f32,
    width: u32,
    height: u32,
    step: u32,
    number_of_frames: u32,
}

impl Serialize for AnimatedSprite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let data = AnimatedSpriteData {
            sheet_id: self.sheet_id,
            index: self.index as u32,
            row: self.row,
            width: self.width as u32,
            height: self.height as u32,
            step: self.step,
            number_of_frames: self.number_of_frames,
        };
        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AnimatedSprite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let data = AnimatedSpriteData::deserialize(deserializer)?;
        let sprite = if data.step == 0 {
            AnimatedSprite::new(data.sheet_id, data.number_of_frames, data.width, data.height)
        } else {
            AnimatedSprite::new_stepped(data.sheet_id, data.number_of_frames, data.index, data.step, data.width, data.height)
        };
        Ok(sprite)
    }
}
