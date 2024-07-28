#[derive(Clone, Copy)]
pub enum AnimationCurve {
    Linear,
}

pub struct Animator {
    initial_value: f32,
    final_value: f32,
    current_value: f32,
    increment_per_second: f32,
    is_active: bool,
    animation_curve: AnimationCurve,
}

impl Animator {
    pub fn new() -> Self {
        Animator {
            initial_value: 0.0,
            final_value: 0.0,
            current_value: 0.0,
            increment_per_second: 0.0,
            is_active: false,
            animation_curve: AnimationCurve::Linear,
        }
    }

    pub fn animate(&mut self, initial_value: f32, final_value: f32, duration: f32, animation_curve: AnimationCurve) {
        self.current_value = initial_value;
        self.initial_value = initial_value;
        self.final_value = final_value;
        self.animation_curve = animation_curve;

        let diff = final_value - initial_value;
        self.increment_per_second = diff / duration;

        self.is_active = true;
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        if !self.is_active {
            return;
        }

        self.current_value += time_since_last_update * self.increment_per_second;

        if self.current_value >= self.final_value {
            self.current_value = self.final_value;
            self.is_active = false;
        }
    }

    pub fn current(&self) -> f32 {
        self.current_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MARGIN: f32 = 0.000001;

    #[test]
    fn test_can_initialize_animator() {
        let mut animator = Animator::new();

        animator.animate(0.0, 1.0, 10.0, AnimationCurve::Linear);

        assert_eq!(animator.current(), 0.0);
    }

    #[test]
    fn test_can_animate_value_linearly() {
        let mut animator = Animator::new();

        animator.animate(0.0, 1.0, 10.0, AnimationCurve::Linear);

        assert_eq!(animator.current(), 0.0);
        animator.update(1.0);
        assert!((animator.current() - 0.1).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.2).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.3).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.4).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.5).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.6).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.7).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.8).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.9).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 1.0).abs() < MARGIN);
    }

    #[test]
    fn test_value_does_not_go_over_final_value() {
        let mut animator = Animator::new();

        animator.animate(0.0, 1.0, 10.0, AnimationCurve::Linear);

        assert_eq!(animator.current(), 0.0);
        animator.update(1.0);
        assert!((animator.current() - 0.1).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.2).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.3).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.4).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.5).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.6).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.7).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.8).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 0.9).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 1.0).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 1.0).abs() < MARGIN);
        animator.update(1.0);
        assert!((animator.current() - 1.0).abs() < MARGIN);
    }
}
