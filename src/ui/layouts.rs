use std::collections::HashMap;

use raylib::prelude::*;

use crate::utils::{rect::Rect, vector::Vector2d};

use super::components::{RenderingConfig, View};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AnchorPoint {
    Center,
    TopLeft,
    TopRight,
    BottomCenter,
}

pub struct Layout {
    pub frame: Rect,
    pub children: HashMap<AnchorPoint, Vec<View>>,
}

impl Layout {
    pub fn new(w: i32, h: i32, children: HashMap<AnchorPoint, Vec<View>>) -> Self {
        Self { 
            frame: Rect::new(0, 0, w, h), 
            children 
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, config: &RenderingConfig) {
        for (anchor, views) in &self.children {
            for view in views {
                let position = self.calculate_position(anchor, view, config);
                view.render(d, config, &position);
            }
        }
    }

    fn calculate_position(&self, anchor: &AnchorPoint, view: &View, config: &RenderingConfig) -> Vector2d {
        let size = view.calculate_size(config);

        let (x, y) = match anchor {
            AnchorPoint::Center => (
                (self.frame.x as f32 + self.frame.w as f32 - size.x) / 2.0, 
                (self.frame.y as f32 + self.frame.w as f32 - size.y) / 2.0
            ), 
            AnchorPoint::TopLeft => (0.0, 0.0), 
            AnchorPoint::TopRight => (
                self.frame.x as f32 + self.frame.w as f32 - size.x, 
                0.0
            ),
            AnchorPoint::BottomCenter => (
                self.frame.x as f32 + self.frame.w as f32 / 2.0 - size.x / 2.0, 
                self.frame.y as f32 + self.frame.h as f32 - size.y
            ),
        };
        Vector2d::new(x, y)
    }
}