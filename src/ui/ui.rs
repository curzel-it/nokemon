use std::collections::HashMap;

use raylib::prelude::*;

pub struct RenderingConfig {
    pub font: Font,
    pub font_bold: Font,
    pub textures: HashMap<String, Texture2D>,
    pub rendering_scale: f32,
}

pub enum TextStyle {
    Bold,
    Regular,
}

pub enum Spacing {
    ZERO,
    SM, 
    MD,
    LG
}

pub enum View {
    ZStack { spacing: Spacing, background_color: Color, children: Vec<View> },
    VStack { spacing: Spacing, children: Vec<View> },
    HStack { spacing: Spacing, children: Vec<View> },
    Text { style: TextStyle, text: String },
    Texture { key: String, source_rect: Rectangle, size: Vector2 },
    Spacing { size: Spacing },
}

#[macro_export]
macro_rules! zstack {
    ($spacing:expr, $background_color:expr, $( $child:expr ),* ) => {
        crate::ui::ui::View::ZStack {
            spacing: $spacing,
            background_color: $background_color,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! vstack {
    ($spacing:expr, $( $child:expr ),* ) => {
        crate::ui::ui::View::VStack {
            spacing: $spacing,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! hstack {
    ($spacing:expr, $( $child:expr ),* ) => {
        crate::ui::ui::View::HStack {
            spacing: $spacing,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! text {
    ($style:expr, $text:expr ) => {
        crate::ui::ui::View::Text {
            style: $style,
            text: $text,
        }
    };
}

#[macro_export]
macro_rules! texture {
    ($key:expr, $source_rect:expr, $size:expr) => {
        crate::ui::ui::View::Texture {
            key: $key,
            source_rect: $source_rect,
            size: $size,
        }
    };
}

#[macro_export]
macro_rules! spacing {
    ($size:expr) => {
        crate::ui::ui::View::Spacing {
            size: $size,
        }
    };
}

pub fn render(view: View, d: &mut RaylibDrawHandle, config: &RenderingConfig, position: Vector2) {
    view.render(d, config, position);
}

impl Spacing {
    fn value(&self) -> f32 {
        match self {
            Spacing::ZERO => 0.0,
            Spacing::SM => 8.0,
            Spacing::MD => 16.0,
            Spacing::LG => 24.0,
        }
    }
}

impl RenderingConfig {
    fn font(&self, style: &TextStyle) -> &Font {
        match style {
            TextStyle::Bold => &self.font_bold,
            TextStyle::Regular => &self.font,
        }
    }

    pub fn get_texture(&self, key: &str) -> Option<&Texture2D> {
        self.textures.get(key)
    }
}

impl View {
    fn render(&self, d: &mut RaylibDrawHandle, config: &RenderingConfig, position: Vector2) {
        match self {
            View::ZStack { spacing, background_color, children } => {
                self.render_zstack(d, config, position, children, spacing, *background_color);
            }
            View::VStack { spacing, children } => {
                self.render_vstack(d, config, position, children, spacing);
            }
            View::HStack { spacing, children } => {
                self.render_hstack(d, config, position, children, spacing);
            }
            View::Text { style, text } => {
                self.render_text(d, config, position, style, text);
            }
            View::Texture { key, source_rect, size } => {
                self.render_texture(d, config, key, source_rect, &position, size);
            }
            View::Spacing { size: _ } => {
                // Not visible
            }
        }
    }

    fn render_zstack(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: Vector2,
        children: &Vec<View>,
        spacing: &Spacing,
        background_color: Color,
    ) {
        let size = self.calculate_size(config);
        d.draw_rectangle_v(position, size, background_color);

        let mut child_position = position;
        child_position.x += spacing.value();
        child_position.y += spacing.value(); 

        for child in children {
            child.render(d, config, child_position);
            child_position.y += child.calculate_size(config).y + spacing.value();
        }
    }

    fn render_vstack(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: Vector2,
        children: &Vec<View>,
        spacing: &Spacing,
    ) {
        let space = spacing.value();
        let mut child_position = position;

        for child in children {
            child.render(d, config, child_position);
            child_position.y += child.calculate_size(config).y + space;
        }
    }

    fn render_hstack(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: Vector2,
        children: &Vec<View>,
        spacing: &Spacing,
    ) {
        let space = spacing.value();
        let mut child_position = position;

        for child in children {
            child.render(d, config, child_position);
            child_position.x += child.calculate_size(config).x + space;
        }
    }

    fn render_text(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: Vector2,
        style: &TextStyle,
        text: &String,
    ) {
        let font = config.font(style);
        d.draw_text_ex(font, text, position, 20.0, 1.0, Color::WHITE);
    }

    fn render_texture(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        key: &String,
        source_rect: &Rectangle,
        position: &Vector2,
        size: &Vector2
    ) {
        if let Some(texture) = config.get_texture(key) {
            // d.draw_rectangle(position.x as i32, position.y as i32, size.x as i32, size.y as i32, Color::RED);
            
            d.draw_texture_pro(
                texture,
                source_rect,
                Rectangle::new(position.x, position.y, size.x, size.y),
                Vector2::zero(), 
                0.0,
                Color::WHITE,
            ); 
        }
    }

    fn calculate_size(&self, config: &RenderingConfig) -> Vector2 {
        match self {
            View::ZStack { spacing, background_color: _, children } => {
                self.calculate_zstack_size(config, children, spacing)
            }
            View::VStack { spacing, children } => {
                self.calculate_vstack_size(config, children, spacing)
            }
            View::HStack { spacing, children } => {
                self.calculate_hstack_size(config, children, spacing)
            }
            View::Text { style, text } => {
                self.calculate_text_size(config, style, text)
            }
            View::Texture { key: _, source_rect: _, size } => {
                size.clone()
            }
            View::Spacing { size } => {
                Vector2::new(size.value(), size.value())
            }
        }
    }

    fn calculate_zstack_size(
        &self,
        config: &RenderingConfig,
        children: &Vec<View>,
        spacing: &Spacing,
    ) -> Vector2 {
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(config);
            max_width = max_width.max(size.x);
            max_height = max_height.max(size.y);
        }
        Vector2::new(
            max_width + spacing.value() * 2.0, 
            max_height + spacing.value() * 2.0
        )
    }

    fn calculate_vstack_size(
        &self,
        config: &RenderingConfig,
        children: &Vec<View>,
        spacing: &Spacing,
    ) -> Vector2 {
        let space = spacing.value();
        let mut total_height: f32 = 0.0;
        let mut max_width: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(config);
            total_height += size.y + space;
            max_width = max_width.max(size.x);
        }
        if children.len() > 0 {
            total_height -= space;
        }
        Vector2::new(max_width, total_height)
    }

    fn calculate_hstack_size(
        &self,
        config: &RenderingConfig,
        children: &Vec<View>,
        spacing: &Spacing,
    ) -> Vector2 {
        let space = spacing.value();
        let mut total_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(config);
            total_width += size.x + space;
            max_height = max_height.max(size.y);
        }
        if children.len() > 0 {
            total_width -= space;
        }
        Vector2::new(total_width, max_height)
    }

    fn calculate_text_size(
        &self,
        config: &RenderingConfig,
        style: &TextStyle,
        text: &String,
    ) -> Vector2 {
        let font = config.font(style);
        let size = font.measure_text(text, 20.0, 1.0);
        Vector2::new(size.x, size.y)
    }
}
