pub struct TimedContentProvider<T> {
    frames: Vec<T>,
    frame_duration: u64,
    current_frame_index: usize,
    completed_loops: u32,
    leftover: u64
}

impl<T> TimedContentProvider<T> {
    pub fn new(frames: Vec<T>, fps: f32) -> Self {
        let frame_duration = if fps > 0.0 {
            (1000.0 / fps) as u64
        } else {
            0
        };
        Self {
            frames,
            frame_duration,
            current_frame_index: 0,
            completed_loops: 0,
            leftover: 0
        }
    }

    pub fn number_of_frames(&self) -> usize {
        self.frames.len()
    }

    pub fn current_frame(&self) -> &T {
        &self.frames[self.current_frame_index]
    }

    pub fn update(&mut self, time_since_last_update: u64) {
        self.leftover += time_since_last_update;

        if self.leftover >= self.frame_duration {
            self.leftover -= self.frame_duration;
            self.load_next_frame();
        }
    }

    fn load_next_frame(&mut self) {
        let next_index = (self.current_frame_index + 1) % self.frames.len();
        self.check_loop_completion(next_index);
        self.current_frame_index = next_index;
    }

    fn check_loop_completion(&mut self, next_index: usize) {
        if next_index < self.current_frame_index {
            self.completed_loops += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_frame() {
        let provider = TimedContentProvider::new(vec![10, 20, 30], 1.0);
        assert_eq!(*provider.current_frame(), 10);
    }

    #[test]
    fn next_frame_advance() {
        let mut provider = TimedContentProvider::new(vec![10, 20, 30], 1.0);
        
        provider.update(500);
        assert_eq!(*provider.current_frame(), 10);
        
        provider.update(500);
        assert_eq!(*provider.current_frame(), 20);
        
        provider.update(1000);
        assert_eq!(*provider.current_frame(), 30);
    }

    #[test]
    fn insufficient_time_does_not_advance_frame() {
        let mut provider = TimedContentProvider::new(vec![10, 20, 30], 1.0);
        
        provider.update(300);
        assert_eq!(*provider.current_frame(), 10);
        
        provider.update(300);
        assert_eq!(*provider.current_frame(), 10);
        
        provider.update(300);
        assert_eq!(*provider.current_frame(), 10);
        
        provider.update(300);
        assert_eq!(*provider.current_frame(), 20);
        
        provider.update(1000);
        assert_eq!(*provider.current_frame(), 30);
    }
}
