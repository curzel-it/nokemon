use std::collections::VecDeque;

use raylib::color::Color;

use crate::{constants::SPRITE_SHEET_MENU, features::animated_sprite::AnimatedSprite, hstack, text, texture, ui::{components::{empty_view, BordersTextures, Spacing, TextureInfo, Typography, View}, scaffold::scaffold}, utils::{animator::Animator, rect::Rect, vector::Vector2d}};

#[derive(Debug, Clone, Copy)]
pub enum ToastMode {
    Regular,
    Important
}

#[derive(Debug, Clone, Copy)]
pub struct ToastImage {
    pub sprite_frame: Rect,
    pub sprite_sheet_id: u32,
    pub number_of_frames: i32,    
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub text: String,
    pub mode: ToastMode,
    pub image: Option<ToastImage>
}

pub struct ToastDisplay {
    animator: Animator,
    text: String,
    mode: ToastMode,
    sprite: Option<AnimatedSprite>,
    queue: VecDeque<Toast>,
}

impl ToastDisplay {
    pub fn new() -> Self {
        Self {
            animator: Animator::new(),
            text: "".to_string(),
            mode: ToastMode::Regular,
            sprite: None,
            queue: VecDeque::new(),
        }
    }

    pub fn show(&mut self, toast: &Toast) {
        if self.animator.is_active {
            if self.text == toast.text {
                return;
            }
            if self.queue.iter().any(|queued| queued.text == toast.text) {
                return;
            }
            self.queue.push_back(toast.clone());
        } else {
            self.show_now(toast.clone());
        }
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.animator.update(time_since_last_update);
        self.sprite.as_mut().map(|sprite| sprite.update(time_since_last_update));

        if !self.animator.is_active && !self.queue.is_empty() {
            if let Some(toast) = self.queue.pop_front() {
                self.show_now(toast);
            }
        }
    }

    fn show_now(&mut self, toast: Toast) {
        self.animator.animate(0.0, 1.0, 2.5);
        self.text = toast.text;
        self.mode = toast.mode;

        if let Some(image) = toast.image {
            self.sprite = Some(AnimatedSprite::new(image.sprite_sheet_id, image.sprite_frame, image.number_of_frames));
        } else {
            self.sprite = None;
        }
        
    }
}

impl ToastImage {
    pub fn new(sprite_frame: Rect, sprite_sheet_id: u32, number_of_frames: i32) -> Self {
        Self {
            sprite_frame,
            sprite_sheet_id,
            number_of_frames
        }
    }
    
    pub fn static_image(sprite_frame: Rect, sprite_sheet_id: u32) -> Self {
        Self::new(sprite_frame, sprite_sheet_id, 1)
    }
}

impl Toast {
    pub fn regular(text: String) -> Self {
        Toast { text, mode: ToastMode::Regular, image: None }
    }
    
    pub fn important_with_image(text: String, image: ToastImage) -> Self {
        Toast { text, mode: ToastMode::Important, image: Some(image) }
    }
}

impl ToastDisplay {
    pub fn important_toast_ui(&self) -> View { 
        if matches!(self.mode, ToastMode::Important) {       
            self.ui()
        } else {
            empty_view()
        }
    }

    pub fn regular_toast_ui(&self) -> View { 
        if matches!(self.mode, ToastMode::Regular) {            
            self.ui()
        } else {
            empty_view()
        }
    }

    fn ui(&self) -> View { 
        if self.animator.is_active {            
            scaffold(
                false, 
                self.background_color(), 
                Some(self.border_texture()),
                self.content()
            )
        } else {
            empty_view()
        }
    }

    fn content(&self) -> View {
        let text = text!(Typography::Regular, self.text.clone());

        if let Some(sprite) = &self.sprite {
            let image = texture!(sprite.sheet_id, sprite.frame, Vector2d::new(2.0, 2.0));
            hstack!(Spacing::MD, image, text)
        } else {
            text
        }
    }

    fn border_texture(&self) -> BordersTextures {
        if matches!(self.mode, ToastMode::Important) {
            TOAST_IMPORTANT_BORDERS_TEXTURES
        } else {
            TOAST_BORDERS_TEXTURES
        }
    }

    fn background_color(&self) -> Color {
        if self.animator.current_value < 0.05 {
            let alpha = 1.0 - (0.05 - self.animator.current_value) * 20.0;
            Color::BLACK.alpha(alpha)
        } else if self.animator.current_value < 0.95 {
            Color::BLACK
        } else {
            let alpha = (1.0 - self.animator.current_value) * 20.0;
            Color::BLACK.alpha(alpha)
        }        
    }
}

const TOAST_BORDERS_TEXTURES: BordersTextures = BordersTextures {
    corner_top_left:     TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 3, y: 0, w: 1, h: 1 } },
    corner_top_right:    TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 5, y: 0, w: 1, h: 1 } },
    corner_bottom_right: TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 5, y: 2, w: 1, h: 1 } },
    corner_bottom_left:  TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 3, y: 2, w: 1, h: 1 } },
    side_top:            TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 4, y: 0, w: 1, h: 1 } },
    side_right:          TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 5, y: 1, w: 1, h: 1 } },
    side_bottom:         TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 4, y: 2, w: 1, h: 1 } },
    side_left:           TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 3, y: 1, w: 1, h: 1 } },
};

const TOAST_IMPORTANT_BORDERS_TEXTURES: BordersTextures = BordersTextures {
    corner_top_left:     TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 6, y: 0, w: 1, h: 1 } },
    corner_top_right:    TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 8, y: 0, w: 1, h: 1 } },
    corner_bottom_right: TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 8, y: 2, w: 1, h: 1 } },
    corner_bottom_left:  TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 6, y: 2, w: 1, h: 1 } },
    side_top:            TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 7, y: 0, w: 1, h: 1 } },
    side_right:          TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 8, y: 1, w: 1, h: 1 } },
    side_bottom:         TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 7, y: 2, w: 1, h: 1 } },
    side_left:           TextureInfo { key: SPRITE_SHEET_MENU, source_rect: Rect { x: 6, y: 1, w: 1, h: 1 } },
};