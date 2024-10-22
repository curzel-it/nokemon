use std::collections::HashMap;

use crate::utils::rect::Rect;

use super::components::View;

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
}