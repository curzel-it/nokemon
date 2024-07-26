use std::string::String;
use std::vec::Vec;

use super::timed_content_provider::TimedContentProvider;

pub struct Sprite {
    pub animation_name: String,
    timed_content_provider: TimedContentProvider<String>,
}

impl Sprite {
    pub fn new(animation_name: String, frames: Vec<String>, fps: f32) -> Self {
        Self {
            animation_name,
            timed_content_provider: TimedContentProvider::new(frames, fps),
        }
    }

    pub fn current_frame(&self) -> &String {
        self.timed_content_provider.current_frame()
    }

    pub fn number_of_frames(&self) -> usize {
        self.timed_content_provider.number_of_frames()
    }

    pub fn update(&mut self, time_since_last_update: u64) {
        self.timed_content_provider.update(time_since_last_update);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_frame() {
        let sprite = Sprite::new(String::from(""), vec![String::from("10"), String::from("20"), String::from("30")], 1.0);
        assert_eq!(sprite.current_frame(), "10");
    }

    #[test]
    fn next_frame_advance() {
        let mut sprite = Sprite::new(String::from(""), vec![String::from("10"), String::from("20"), String::from("30")], 1.0);

        sprite.update(500);
        assert_eq!(sprite.current_frame(), "10");

        sprite.update(500);
        assert_eq!(sprite.current_frame(), "20");

        sprite.update(1000);
        assert_eq!(sprite.current_frame(), "30");
    }

    #[test]
    fn next_frame_with_insufficient_time_does_not_advance() {
        let mut sprite = Sprite::new(String::from(""), vec![String::from("10"), String::from("20"), String::from("30")], 1.0);

        sprite.update(300);
        assert_eq!(sprite.current_frame(), "10");

        sprite.update(300);
        assert_eq!(sprite.current_frame(), "10");

        sprite.update(300);
        assert_eq!(sprite.current_frame(), "10");

        sprite.update(300);
        assert_eq!(sprite.current_frame(), "20");

        sprite.update(1000);
        assert_eq!(sprite.current_frame(), "30");
    }
}
