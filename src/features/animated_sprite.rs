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