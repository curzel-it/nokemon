use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{constants::{ANIMATIONS_FPS, SPRITE_SHEET_HUMANOIDS_1X1, SPRITE_SHEET_HUMANOIDS_1X2, SPRITE_SHEET_HUMANOIDS_2X2, UNLIMITED_LIFESPAN}, game_engine::entity::Entity, utils::{directions::Direction, rect::Rect, timed_content_provider::TimedContentProvider}};

#[derive(Debug, Clone)]
pub struct AnimatedSprite {
    pub sheet_id: u32, 
    pub frame: Rect,
    pub supports_directions: bool,
    original_frame: Rect,
    number_of_frames: i32,
    frames_provider: TimedContentProvider<i32>,
}

impl AnimatedSprite {
    pub fn new(sheet_id: u32, frame: Rect, number_of_frames: i32) -> Self {
        Self {
            sheet_id, 
            frame,
            supports_directions: supports_directions(sheet_id),
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
    pub fn update_sprite_for_current_state(&mut self) {
        if self.remaining_lifespan == UNLIMITED_LIFESPAN || self.remaining_lifespan > 1.0 {
            if self.demands_attention {
                self.sprite.frame.y = self.sprite.frame.h * 8
            } else {
                self.update_sprite_for_direction_speed(self.direction, self.current_speed)
            }
        }
    }

    pub fn update_sprite_for_direction_speed(&mut self, direction: Direction, speed: f32) {
        let row = match (direction, speed != 0.0) {
            (Direction::Up, true) => 0,
            (Direction::Up, false) => 1,
            (Direction::Right, true) => 2,
            (Direction::Right, false) => 3,
            (Direction::Down, true) => 4,
            (Direction::Down, false) => 5,
            (Direction::Left, true) => 6,
            (Direction::Left, false) => 7,
            (Direction::Unknown, true) => 4,
            (Direction::Unknown, false) => 5,
            (Direction::Still, true) => 4,
            (Direction::Still, false) => 5,
        };
        self.sprite.frame.y = self.sprite.frame.h * row;
    }
}

impl TimedContentProvider<i32> {
    pub fn frames(x: i32, n: i32, w: i32) -> Self {
        let frames = (0..n).map(|i| x + i * w).collect();
        Self::new(frames, ANIMATIONS_FPS)
    }
}

fn supports_directions(sheet_id: u32) -> bool {
    match sheet_id {
        SPRITE_SHEET_HUMANOIDS_1X1 => true,
        SPRITE_SHEET_HUMANOIDS_1X2 => true,
        SPRITE_SHEET_HUMANOIDS_2X2 => true,
        SPRITE_SHEET_HUMANOIDS_2X3 => true,
        _ => false
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