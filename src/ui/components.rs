use std::collections::HashMap;

use raylib::prelude::*;

use crate::{constants::TILE_SIZE, utils::{rect::Rect, vector::Vector2d}};

pub struct RenderingConfig {
    pub font: Font,
    pub font_bold: Font,
    pub textures: HashMap<u32, Texture2D>,
    pub rendering_scale: f32,
    pub font_rendering_scale: f32,
    pub canvas_size: Vector2d
}

#[derive(Copy, Clone, Debug)]
pub enum Typography {
    Title,
    Regular,
    Selected,
}

pub enum Spacing {
    Zero,
    XS, 
    SM, 
    MD,
    LG,
    XL,
    Custom(f32),
    TextLine(Typography),
}

pub struct GridSpacing {
    between_columns: Spacing,
    between_rows: Spacing,
}

pub enum AnchorPoint {
    TopLeft,
    TopRight,
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

pub enum View {
    ZStack { spacing: Spacing, background_color: Color, children: Vec<View> },
    VStack { spacing: Spacing, children: Vec<View> },
    HStack { spacing: Spacing, children: Vec<View> },
    Text { style: Typography, text: String },
    Texture { key: u32, source_rect: Rect, size: Vector2d },
    Spacing { size: Spacing },
    VGrid { columns: usize, spacing: GridSpacing, children: Vec<View> },
    FullScreenBackdrop { children: Vec<View> },
    FixedPosition { position: Vector2d, children: Vec<View> },
    TexturedBorder { borders: BordersTextures, children: Vec<View> },
    ProgressBar { foreground: Color, background: Color, value: f32 }
}

pub struct TextureInfo {
    pub key: u32,
    pub source_rect: Rect
}

pub struct BordersTextures {
    pub corner_top_left: TextureInfo,
    pub corner_top_right: TextureInfo,
    pub corner_bottom_right: TextureInfo,
    pub corner_bottom_left: TextureInfo,
    pub side_top: TextureInfo,
    pub side_right: TextureInfo,
    pub side_bottom: TextureInfo,
    pub side_left: TextureInfo,
}

#[macro_export]
macro_rules! zstack {
    ($spacing:expr, $background_color:expr, $( $child:expr ),*) => {
        $crate::ui::components::View::ZStack {
            spacing: $spacing,
            background_color: $background_color,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! vstack {
    ($spacing:expr, $( $child:expr ),*) => {
        $crate::ui::components::View::VStack {
            spacing: $spacing,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! hstack {
    ($spacing:expr, $( $child:expr ),*) => {
        $crate::ui::components::View::HStack {
            spacing: $spacing,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! text {
    ($style:expr, $text:expr) => {
        $crate::ui::components::View::Text {
            style: $style,
            text: $text,
        }
    };
}

#[macro_export]
macro_rules! texture {
    ($key:expr, $source_rect:expr, $size:expr) => {
        $crate::ui::components::View::Texture {
            key: $key,
            source_rect: $source_rect,
            size: $size,
        }
    };
}

#[macro_export]
macro_rules! spacing {
    ($size:expr) => {
        $crate::ui::components::View::Spacing {
            size: $size,
        }
    };
}

#[macro_export]
macro_rules! vgrid {
    ($columns:expr, $spacing:expr, $( $child:expr ),*) => {
        $crate::ui::components::View::VGrid {
            columns: $columns,
            spacing: $spacing,
            children: vec![$($child),*],
        }
    };
}

pub fn padding(padding: Spacing, content: View) -> View {
    zstack!(padding, Color::BLACK.alpha(0.0), content)
}

pub fn with_backdrop(content: View) -> View {
    View::FullScreenBackdrop { children: vec![content] }
}

pub fn with_fixed_position(position: Vector2d, content: View) -> View {
    View::FixedPosition { position, children: vec![content] }
}

pub fn with_textured_border(borders: BordersTextures, content: View) -> View {
    View::TexturedBorder { borders, children: vec![content] }
}

pub fn empty_view() -> View {
    with_fixed_position(Vector2d::zero(), spacing!(Spacing::Zero))
}

pub fn render_from(anchor_point: AnchorPoint, view: &View, d: &mut RaylibDrawHandle, config: &RenderingConfig, position: &Vector2d) {
    view.render_from(d, config, position, anchor_point);
}

impl GridSpacing {
    pub fn new(between_rows: Spacing, between_columns: Spacing) -> Self {
        Self {
            between_rows,
            between_columns, 
        }
    }

    pub fn sm() -> Self {
        Self::new(Spacing::SM, Spacing::SM)
    }
}

impl RenderingConfig {
    pub fn scaled_font_size(&self, style: &Typography) -> f32 {
        self.font_rendering_scale * match style {
            Typography::Title => 12.0,
            Typography::Selected => 8.0,
            Typography::Regular => 8.0,
        }
    }

    pub fn scaled_font_spacing(&self, _: &Typography) -> f32 {
        0.0 
    }

    pub fn font_lines_spacing(&self, style: &Typography) -> f32 {
        self.scaled_font_size(style) / 2.0
    }
}

impl Spacing {
    pub fn unscaled_value(&self) -> f32 {
        match self {
            Spacing::Zero => 0.0,
            Spacing::XS => 2.0,
            Spacing::SM => 4.0,
            Spacing::MD => 8.0,
            Spacing::LG => 12.0,
            Spacing::XL => 20.0,
            Spacing::Custom(value) => *value,
            Spacing::TextLine(_) => 4.0,
        }
    }

    pub fn value(&self, config: &RenderingConfig) -> f32 {
        match self {
            Spacing::TextLine(style) => {
                config.rendering_scale * config.font_lines_spacing(style)
            },
            _ => config.rendering_scale * self.unscaled_value()
        }        
    }
}

impl RenderingConfig {
    fn text_color(&self, style: &Typography) -> &Color {
        match style {
            Typography::Selected => &Color::YELLOW,
            _ => &Color::WHITE
        }
    }

    pub fn font(&self, style: &Typography) -> &Font {
        match style {
            Typography::Title => &self.font_bold,
            Typography::Selected => &self.font_bold,
            Typography::Regular => &self.font,
        }
    }

    pub fn get_texture(&self, key: u32) -> Option<&Texture2D> {
        self.textures.get(&key)
    }
}

impl View {
    fn accounts_for_stack_size(&self) -> bool {
        !matches!(self, View::FixedPosition { position: _, children: _})
    }

    fn render_from(
        &self, 
        d: &mut RaylibDrawHandle, 
        config: &RenderingConfig, 
        position: &Vector2d,
        anchor_point: AnchorPoint
    ) {
        if let AnchorPoint::TopLeft = anchor_point {
            return self.render(d, config, position);
        }
        let size = self.calculate_size(config);

        let (x, y) = match anchor_point {
            AnchorPoint::TopLeft => (position.x, position.y),
            AnchorPoint::TopRight => (position.x - size.x, position.y),
            AnchorPoint::Center => (position.x - size.x / 2.0, position.y - size.y / 2.0),
            AnchorPoint::BottomRight => (position.x - size.x, position.y - size.y),
            AnchorPoint::BottomCenter => (position.x - size.x / 2.0, position.y - size.y),
            AnchorPoint::BottomLeft => (position.x, position.y - size.y)
        };

        let real_position = Vector2d::new(x, y);
        self.render(d, config, &real_position)
    }

    fn render(
        &self, 
        d: &mut RaylibDrawHandle, 
        config: &RenderingConfig, 
        position:  &Vector2d
    ) {
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
                self.render_texture(d, config, key, source_rect, position, size);
            }
            View::Spacing { size: _ } => {
                // Not visible
            }
            View::VGrid { columns, spacing, children } => {
                self.render_vgrid(d, config, position, columns, spacing, children);
            }
            View::FullScreenBackdrop { children } => {
                self.render_fullscreen_backdrop(d, config, position, children)
            }
            View::FixedPosition { position, children } => {
                self.render_fixed_position(d, config, position, children)
            }
            View::TexturedBorder { borders, children } => {
                self.render_textured_borders(d, config, borders, position, children)
            }
            View::ProgressBar { foreground, background, value } => {
                self.render_progress_bar(d, config, position, foreground, background, value)
            }
        }
    }

    fn render_progress_bar(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        foreground: &Color,
        background: &Color,
        value: &f32,
    ) {
        let size = self.calculate_size(config);
        let fg_size = Vector2d::new(size.x * value, size.y);

        d.draw_rectangle_v(position.as_rv(), size.as_rv(), background);
        d.draw_rectangle_v(position.as_rv(), fg_size.as_rv(), foreground);
    }

    fn render_textured_borders(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        borders: &BordersTextures,
        position: &Vector2d,
        children: &[View]
    ) {
        let content_size = self.calculate_zstack_size(config, children, &Spacing::Zero);
        let base_texture_size = self.calculate_texture_size(config, &Vector2d::new(1.0, 1.0)).x;
        let size_one = Vector2d::new(1.0, 1.0);

        let top_left = Vector2d { 
            x: position.x - base_texture_size / 2.0, 
            y: position.y - base_texture_size / 2.0, 
        };
        let top_right = Vector2d { 
            x: position.x + content_size.x - base_texture_size / 2.0, 
            y: position.y - base_texture_size / 2.0, 
        };
        let bottom_right = Vector2d { 
            x: position.x + content_size.x - base_texture_size / 2.0,
            y: position.y + content_size.y - base_texture_size / 2.0,
        };
        let bottom_left = Vector2d { 
            x: position.x - base_texture_size / 2.0, 
            y: position.y + content_size.y - base_texture_size / 2.0,
        };
        let side_top = Vector2d { 
            x: position.x + base_texture_size / 2.0, 
            y: position.y - base_texture_size / 2.0, 
        };
        let side_right = Vector2d { 
            x: position.x + content_size.x - base_texture_size / 2.0, 
            y: position.y + base_texture_size / 2.0, 
        };
        let side_bottom = Vector2d { 
            x: position.x + base_texture_size / 2.0, 
            y: position.y + content_size.y - base_texture_size / 2.0, 
        };
        let side_left = Vector2d { 
            x: position.x - base_texture_size / 2.0, 
            y: position.y + base_texture_size / 2.0, 
        };
        let side_horizontal_size = Vector2d { 
            x: (content_size.x / config.rendering_scale) / TILE_SIZE - 1.0, 
            y: 1.0
        };
        let side_vertical_size = Vector2d { 
            x: 1.0, 
            y: (content_size.y / config.rendering_scale) / TILE_SIZE - 1.0, 
        };

        self.render_zstack(d, config, position, children, &Spacing::Zero, Color::RED.alpha(0.0));        
        self.render_texture(d, config, &borders.corner_top_left.key, &borders.corner_top_left.source_rect, &top_left, &size_one);
        self.render_texture(d, config, &borders.corner_top_right.key, &borders.corner_top_right.source_rect, &top_right, &size_one);
        self.render_texture(d, config, &borders.corner_bottom_right.key, &borders.corner_bottom_right.source_rect, &bottom_right, &size_one);
        self.render_texture(d, config, &borders.corner_bottom_left.key, &borders.corner_bottom_left.source_rect, &bottom_left, &size_one);        
        self.render_texture(d, config, &borders.side_top.key, &borders.side_top.source_rect, &side_top, &side_horizontal_size);
        self.render_texture(d, config, &borders.side_right.key, &borders.side_right.source_rect, &side_right, &side_vertical_size);
        self.render_texture(d, config, &borders.side_bottom.key, &borders.side_bottom.source_rect, &side_bottom, &side_horizontal_size);
        self.render_texture(d, config, &borders.side_left.key, &borders.side_left.source_rect, &side_left, &side_vertical_size);
    }

    fn render_zstack(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        children: &[View],
        spacing: &Spacing,
        background_color: Color,
    ) {
        let space = spacing.value(config);
        let size = self.calculate_size(config);
        let child_position = Vector2d::new(position.x + space, position.y + space);

        d.draw_rectangle_v(position.as_rv(), size.as_rv(), background_color);

        for child in children {
            child.render(d, config, &child_position);
        }
    }

    fn render_vstack(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        children: &[View],
        spacing: &Spacing,
    ) {
        let space = spacing.value(config);
        let mut child_position = *position;

        for child in children {
            child.render(d, config, &child_position);
            if child.accounts_for_stack_size() { 
                child_position.y += child.calculate_size(config).y + space;
            }
        }
    }

    fn render_hstack(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        children: &[View],
        spacing: &Spacing,
    ) {
        let space = spacing.value(config);
        let mut child_position = *position;

        for child in children {
            child.render(d, config, &child_position);
            if child.accounts_for_stack_size() { 
                child_position.x += child.calculate_size(config).x + space;
            }
        }
    }

    fn render_text(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        style: &Typography,
        text: &str,
    ) { 
        if !text.contains("\n") {
            let font = config.font(style);
            let font_size = config.scaled_font_size(style);
            let font_spacing = config.scaled_font_spacing(style);
            let color = config.text_color(style);
            d.draw_text_ex(font, text, position.as_rv(), font_size, font_spacing, color);
        } else {
            let stack = self.multiline_text_to_vstack(style, text);
            stack.render(d, config, position);
        }
    }

    fn multiline_text_to_vstack(&self, style: &Typography, text: &str) -> View {
        let lines = text.split("\n");
        let texts: Vec<View> = lines.map(|line_text|
            View::Text { 
                style: *style, 
                text: line_text.replace("\n", " ").to_string()
            }
        ).collect();
                
        View::VStack { 
            spacing: Spacing::TextLine(*style), 
            children: texts 
        }
    }

    fn render_texture(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        key: &u32,
        source: &Rect,
        position: &Vector2d,
        size:  &Vector2d
    ) {
        if let Some(texture) = config.get_texture(*key) {           
            let source_rect = Rectangle::new(
                TILE_SIZE * source.x as f32, 
                TILE_SIZE * source.y as f32, 
                TILE_SIZE * source.w as f32,
                TILE_SIZE * source.h as f32
            );
            let dest_rect = Rectangle::new(
                position.x, 
                position.y, 
                config.rendering_scale * TILE_SIZE * size.x, 
                config.rendering_scale * TILE_SIZE * size.y
            );
            d.draw_texture_pro(
                texture,
                source_rect, 
                dest_rect,
                Vector2::zero(), 
                0.0,
                Color::WHITE,
            ); 
        }
    }

    fn render_vgrid(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        columns: &usize,
        spacing: &GridSpacing,
        children: &[View],
    ) {
        let row_space: f32 = spacing.between_rows.value(config);
        let mut row_position: Vector2d = *position;        
        let rows = children.chunks(*columns);

        for row in rows {
            self.render_hstack(d, config, &row_position, row, &spacing.between_columns);
            let row_size = self.calculate_hstack_size(config, row, &spacing.between_columns);
            row_position.y += row_size.y + row_space;
        }
    }

    fn render_fullscreen_backdrop(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        children: &[View],
    ) {
        d.draw_rectangle_v(
            Vector2::zero(), 
            config.canvas_size.as_rv(), 
            Color::BLACK.alpha(0.4)
        );
        self.render_zstack(
            d,
            config,
            position,
            children,
            &Spacing::Zero,
            Color::BLACK.alpha(0.0)
        );
    }

    fn render_fixed_position(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        children: &[View],
    ) {
        self.render_zstack(
            d, 
            config, 
            &position.scaled(config.rendering_scale), 
            children, 
            &Spacing::Zero, 
            Color::BLACK.alpha(0.0)
        );
    }
}

impl View {
    fn calculate_size(&self, config: &RenderingConfig) -> Vector2d {
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
                self.calculate_texture_size(config, size)
            }
            View::Spacing { size } => {
                self.calculate_spacing_size(config, size)
            }
            View::VGrid { columns, spacing, children } => {
                self.calculate_vgrid_size(config, columns, spacing, children)
            }
            View::FullScreenBackdrop { children } => {
                self.calculate_fullscreen_backdrop_size(config, children)                
            }
            View::FixedPosition { position: _, children } => {
                self.calculate_fixed_position_size(config, children)                
            }
            View::TexturedBorder { borders: _, children } => {
                self.calculate_textured_border_size(config, children)                
            }
            View::ProgressBar{ foreground: _, background: _, value: _ } => {
                self.calculate_progress_bar_size(config)                
            }
        }
    }

    fn calculate_progress_bar_size(&self, config: &RenderingConfig) -> Vector2d {
        Vector2d::new(160.0, 10.0).scaled(config.rendering_scale)
    }

    fn calculate_textured_border_size(&self, config: &RenderingConfig, children: &[View]) -> Vector2d {
        self.calculate_zstack_size(config, children, &Spacing::Zero)
    }

    fn calculate_texture_size(&self, config: &RenderingConfig, size: &Vector2d) -> Vector2d {
        Vector2d::new(
            size.x * TILE_SIZE * config.rendering_scale, 
            size.y * TILE_SIZE * config.rendering_scale
        )
    }

    fn calculate_spacing_size(&self, config: &RenderingConfig, size: &Spacing) -> Vector2d {
        Vector2d::new(size.value(config), size.value(config))
    }

    fn calculate_zstack_size(
        &self,
        config: &RenderingConfig,
        children: &[View],
        spacing: &Spacing,
    ) -> Vector2d {
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(config);
            max_width = max_width.max(size.x);
            max_height = max_height.max(size.y);
        }
        Vector2d::new(
            max_width + spacing.value(config) * 2.0, 
            max_height + spacing.value(config) * 2.0
        )
    }

    fn calculate_vstack_size(
        &self,
        config: &RenderingConfig,
        children: &[View],
        spacing: &Spacing,
    ) -> Vector2d {
        let space = spacing.value(config);
        let mut total_height: f32 = 0.0;
        let mut max_width: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(config);
            if child.accounts_for_stack_size() { 
                total_height += size.y + space;
                max_width = max_width.max(size.x);
            }
        }
        if !children.is_empty() {
            total_height -= space;
        }
        Vector2d::new(max_width, total_height)
    }

    fn calculate_hstack_size(
        &self,
        config: &RenderingConfig,
        children: &[View],
        spacing: &Spacing,
    ) -> Vector2d {
        let space = spacing.value(config);
        let mut total_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(config);
            if child.accounts_for_stack_size() { 
                total_width += size.x + space;
                max_height = max_height.max(size.y);
            }
        }
        if !children.is_empty() {
            total_width -= space;
        }
        Vector2d::new(total_width, max_height)
    }

    fn calculate_text_size(
        &self,
        config: &RenderingConfig,
        style: &Typography,
        text: &str,
    ) -> Vector2d {
        if !text.contains("\n") {
            let font = config.font(style);
            let font_size = config.scaled_font_size(style);
            let font_spacing = config.scaled_font_spacing(style);
            let size = font.measure_text(text, font_size, font_spacing);
            Vector2d::new(size.x, size.y)
        } else {
            let stack = self.multiline_text_to_vstack(style, text);
            stack.calculate_size(config)
        }
    }

    fn calculate_vgrid_size(
        &self,
        config: &RenderingConfig,
        columns: &usize,
        spacing: &GridSpacing,
        children: &[View],
    ) -> Vector2d {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        let rows = children.chunks(*columns);
        let rows_count = rows.len();

        for row in rows {
            let row_size = self.calculate_hstack_size(
                config,
                row, 
                &spacing.between_columns
            );
            width = width.max(row_size.x);
            height += row_size.y;
        }

        height += (rows_count.max(1) - 1) as f32 * spacing.between_rows.value(config);
        Vector2d::new(width, height)
    }

    fn calculate_fullscreen_backdrop_size(&self, config: &RenderingConfig, children: &[View]) -> Vector2d {
        self.calculate_zstack_size(config, children, &Spacing::Zero)
    }

    fn calculate_fixed_position_size(&self, config: &RenderingConfig, children: &[View]) -> Vector2d {
        self.calculate_zstack_size(config, children, &Spacing::Zero)
    }
}