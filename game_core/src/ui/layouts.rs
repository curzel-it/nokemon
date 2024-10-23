use crate::utils::rect::Rect;

use super::components::{NonColor, View};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AnchorPoint {
    Center,
    TopLeft,
    TopRight,
    BottomCenter,
}

pub struct Layout {
    pub frame: Rect,
    pub background_color: NonColor,
    pub children: Vec<(AnchorPoint, View)>,
}

impl Layout {
    pub fn new(
        w: i32, 
        h: i32, 
        background_color: NonColor, 
        children: Vec<(AnchorPoint, View)>
    ) -> Self {
        Self { 
            background_color,
            frame: Rect::new(0, 0, w, h), 
            children 
        }
    }
}