use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{constants::ANIMATIONS_FPS, game_engine::entity::Entity, utils::{directions::Direction, rect::Rect, timed_content_provider::TimedContentProvider}};

#[derive(Debug, Clone)]
pub struct AnimatedSprite {
    pub sheet_id: u32, 
    pub frame: Rect,
    original_frame: Rect,
    number_of_frames: i32,
    frames_provider: TimedContentProvider<i32>,
}

impl AnimatedSprite {
    pub fn new(sheet_id: u32, frame: Rect, number_of_frames: i32) -> Self {
        Self {
            sheet_id, 
            frame,
            original_frame: frame,
            number_of_frames,
            frames_provider: TimedContentProvider::frames(frame.x, number_of_frames, frame.w),
        }
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        if self.number_of_frames > 1 {
            self.frames_provider.update(time_since_last_update);
            self.frame.x = *self.frames_provider.current_frame();
        }
    }

    pub fn texture_source_rect(&self) -> Rect {
        self.frame
    }
}

impl Entity {
    pub fn update_sprite_for_current_direction(&mut self) {
        self.sprite.frame.y = match (self.direction, self.current_speed != 0.0) {
            (Direction::Up, true) => 0,
            (Direction::Up, false) => 2,
            (Direction::Right, true) => 4,
            (Direction::Right, false) => 6,
            (Direction::Down, true) => 8,
            (Direction::Down, false) => 10,
            (Direction::Left, true) => 12,
            (Direction::Left, false) => 14,
            (Direction::Unknown, true) => 10,
            (Direction::Unknown, false) => 10
        }
    }
}

impl TimedContentProvider<i32> {
    pub fn frames(x: i32, n: i32, w: i32) -> Self {
        let frames = (0..n).map(|i| x + i * w).collect();
        Self::new(frames, ANIMATIONS_FPS)
    }
}

#[derive(Serialize, Deserialize)]
struct AnimatedSpriteData {
    sheet_id: u32, 
    frame: Rect,
    number_of_frames: i32,
}

impl Serialize for AnimatedSprite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let data = AnimatedSpriteData {
            sheet_id: self.sheet_id, 
            frame: self.original_frame, 
            number_of_frames: self.number_of_frames,
        };
        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AnimatedSprite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let AnimatedSpriteData { sheet_id, frame, number_of_frames } = AnimatedSpriteData::deserialize(deserializer)?;
        let sprite = AnimatedSprite::new(sheet_id, frame, number_of_frames);
        Ok(sprite)
    }
}
