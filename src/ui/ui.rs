use std::collections::HashMap;

use raylib::prelude::*;

use crate::{constants::{SPRITE_SHEET_INVENTORY, TILE_SIZE}, utils::{rect::Rect, vector::Vector2d}};

pub struct RenderingConfig {
    pub font: Font,
    pub font_bold: Font,
    pub textures: HashMap<u32, Texture2D>,
    pub rendering_scale: f32,
    pub font_rendering_scale: f32,
    pub canvas_size: Vector2d
}

#[derive(Copy, Clone, Debug)]
pub enum TextStyle {
    LargeTitle,
    Title,
    Regular,
    Bold
}

pub enum Spacing {
    ZERO,
    XS, 
    SM, 
    MD,
    LG,
    Custom(f32),
    TextLineSpacing(TextStyle)
}

pub struct GridSpacing {
    between_columns: Spacing,
    between_rows: Spacing,
}

pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub enum View {
    ZStack { spacing: Spacing, background_color: Color, children: Vec<View> },
    VStack { spacing: Spacing, children: Vec<View> },
    HStack { spacing: Spacing, children: Vec<View> },
    Text { style: TextStyle, text: String },
    Texture { key: u32, source_rect: Rect, size: Vector2d },
    Spacing { size: Spacing },
    VGrid { columns: usize, spacing: GridSpacing, children: Vec<View> },
    HGrid { rows: usize, spacing: GridSpacing, children: Vec<View> },
    FullScreenBackdrop { children: Vec<View> },
    FixedPosition { position: Vector2d, children: Vec<View> },
}

#[macro_export]
macro_rules! zstack {
    ($spacing:expr, $background_color:expr, $( $child:expr ),*) => {
        $crate::ui::ui::View::ZStack {
            spacing: $spacing,
            background_color: $background_color,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! vstack {
    ($spacing:expr, $( $child:expr ),*) => {
        $crate::ui::ui::View::VStack {
            spacing: $spacing,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! hstack {
    ($spacing:expr, $( $child:expr ),*) => {
        $crate::ui::ui::View::HStack {
            spacing: $spacing,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! text {
    ($style:expr, $text:expr) => {
        $crate::ui::ui::View::Text {
            style: $style,
            text: $text,
        }
    };
}

#[macro_export]
macro_rules! texture {
    ($key:expr, $source_rect:expr, $size:expr) => {
        $crate::ui::ui::View::Texture {
            key: $key,
            source_rect: $source_rect,
            size: $size,
        }
    };
}

#[macro_export]
macro_rules! spacing {
    ($size:expr) => {
        $crate::ui::ui::View::Spacing {
            size: $size,
        }
    };
}

#[macro_export]
macro_rules! vgrid {
    ($columns:expr, $spacing:expr, $( $child:expr ),*) => {
        $crate::ui::ui::View::VGrid {
            columns: $columns,
            spacing: $spacing,
            children: vec![$($child),*],
        }
    };
}

#[macro_export]
macro_rules! hgrid {
    ($rows:expr, $spacing:expr, $( $child:expr ),*) => {
        $crate::ui::ui::View::HGrid {
            rows: $rows,
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

pub fn render(view: View, d: &mut RaylibDrawHandle, config: &RenderingConfig, position: &Vector2d) {
    view.render(d, config, position);
}

pub fn render_from(corner: Corner, view: View, d: &mut RaylibDrawHandle, config: &RenderingConfig, position: &Vector2d) {
    view.render_from(d, config, position, corner);
}

impl GridSpacing {
    pub fn new(between_rows: Spacing, between_columns: Spacing) -> Self {
        Self {
            between_rows,
            between_columns, 
        }
    }

    pub fn ZERO() -> Self {
        Self::new(Spacing::ZERO, Spacing::ZERO)
    }

    pub fn SM() -> Self {
        Self::new(Spacing::SM, Spacing::SM)
    }

    pub fn MD() -> Self {
        Self::new(Spacing::MD, Spacing::MD)
    }

    pub fn LG() -> Self {
        Self::new(Spacing::LG, Spacing::LG)
    }
}

impl RenderingConfig {
    pub fn scaled_font_size(&self, style: &TextStyle) -> f32 {
        self.font_rendering_scale * match style {
            TextStyle::LargeTitle => 12.0,
            TextStyle::Title => 9.0,
            TextStyle::Bold => 7.0,
            TextStyle::Regular => 7.0,
        }
    }

    pub fn scaled_font_spacing(&self, style: &TextStyle) -> f32 {
        self.scaled_font_size(style) / 10.0
    }

    pub fn font_lines_spacing(&self, style: &TextStyle) -> f32 {
        self.scaled_font_size(style) / 3.0
    }
}

impl Spacing {
    pub fn unscaled_value(&self) -> f32 {
        match self {
            Spacing::ZERO => 0.0,
            Spacing::XS => 2.0,
            Spacing::SM => 4.0,
            Spacing::MD => 8.0,
            Spacing::LG => 12.0,
            Spacing::Custom(value) => *value,
            Spacing::TextLineSpacing(_) => 4.0,
        }
    }

    fn value(&self, config: &RenderingConfig) -> f32 {
        match self {
            Spacing::TextLineSpacing(style) => {
                config.rendering_scale * config.font_lines_spacing(style)
            },
            _ => config.rendering_scale * self.unscaled_value()
        }        
    }
}

impl RenderingConfig {
    fn font(&self, style: &TextStyle) -> &Font {
        match style {
            TextStyle::LargeTitle => &self.font_bold,
            TextStyle::Title => &self.font_bold,
            TextStyle::Bold => &self.font_bold,
            TextStyle::Regular => &self.font,
        }
    }

    pub fn get_texture(&self, key: u32) -> Option<&Texture2D> {
        self.textures.get(&key)
    }
}

impl View {
    fn accounts_for_stack_size(&self) -> bool {
        match self {
            View::FixedPosition { position: _, children: _} => false,
            _ => true
        }
    }

    fn render_from(
        &self, 
        d: &mut RaylibDrawHandle, 
        config: &RenderingConfig, 
        position: &Vector2d,
        corner: Corner
    ) {
        if let Corner::TopLeft = corner {
            return self.render(d, config, position);
        }
        let size = self.calculate_size(config);

        let (x, y) = match corner {
            Corner::TopLeft => (position.x, position.y),
            Corner::TopRight => (position.x - size.x, position.y),
            Corner::BottomRight => (position.x - size.x, position.y - size.y),
            Corner::BottomLeft => (position.x, position.y - size.y)
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
            View::HGrid { rows, spacing, children } => {
                self.render_hgrid(d, config, position, rows, spacing, children);
            }
            View::FullScreenBackdrop { children } => {
                self.render_fullscreen_backdrop(d, config, position, children)
            }
            View::FixedPosition { position, children } => {
                self.render_fixed_position(d, config, position, children)
            }
        }
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
        style: &TextStyle,
        text: &String,
    ) { 
        if !text.contains("\n") {
            let font = config.font(style);
            let font_size = config.scaled_font_size(style);
            let font_spacing = config.scaled_font_spacing(style);
            d.draw_text_ex(font, text, position.as_rv(), font_size, font_spacing, Color::WHITE);
        } else {
            let stack = self.multiline_text_to_vstack(style, text);
            stack.render(d, config, position);
        }
    }

    fn multiline_text_to_vstack(&self, style: &TextStyle, text: &String) -> View {
        let lines = text.split("\n");
        let texts: Vec<View> = lines.map(|line_text|
            View::Text { 
                style: *style, 
                text: line_text.replace("\n", " ").to_string()
            }
        ).collect();
                
        View::VStack { 
            spacing: Spacing::TextLineSpacing(*style), 
            children: texts 
        }
    }

    fn render_texture(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        key: &u32,
        source_rect: &Rect,
        position: &Vector2d,
        size:  &Vector2d
    ) {
        if let Some(texture) = config.get_texture(*key) {           
            d.draw_texture_pro(
                texture,
                source_rect.as_rr(),
                Rectangle::new(
                    position.x, 
                    position.y, 
                    config.rendering_scale * size.x, 
                    config.rendering_scale * size.y
                ),
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

    fn render_hgrid(
        &self,
        d: &mut RaylibDrawHandle,
        config: &RenderingConfig,
        position: &Vector2d,
        rows: &usize,
        spacing: &GridSpacing,
        children: &[View],
    ) {
        let column_space: f32 = spacing.between_columns.value(config);
        let mut column_position: Vector2d = *position;        
        let columns = children.chunks(*rows);

        for column in columns {
            self.render_vstack(d, config, &column_position, column, &spacing.between_rows);
            let column_size = self.calculate_vstack_size(config, column, &spacing.between_rows);
            column_position.x += column_size.x + column_space;
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
            &Spacing::ZERO,
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
            &Spacing::ZERO, 
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
            View::HGrid { rows, spacing, children } => {
                self.calculate_hgrid_size(config, rows, spacing, children)
            }
            View::FullScreenBackdrop { children } => {
                self.calculate_fullscreen_backdrop_size(config, children)                
            }
            View::FixedPosition { position: _, children } => {
                self.calculate_fixed_position_size(config, children)                
            }
        }
    }

    fn calculate_texture_size(&self, config: &RenderingConfig, size: &Vector2d) -> Vector2d {
        Vector2d::new(size.x * config.rendering_scale, size.y * config.rendering_scale)
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
        style: &TextStyle,
        text: &String,
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

        height += (rows_count - 1).max(0) as f32 * spacing.between_rows.value(config);
        Vector2d::new(width, height)
    }

    fn calculate_hgrid_size(
        &self,
        config: &RenderingConfig,
        rows: &usize,
        spacing: &GridSpacing,
        children: &[View],
    ) -> Vector2d {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        let columns = children.chunks(*rows);
        let columns_count = columns.len();

        for column in columns {
            let column_size = self.calculate_vstack_size(
                config,
                column, 
                &spacing.between_rows
            );
            height = height.max(column_size.y);
            width += column_size.x;
        }

        width += (columns_count - 1).max(0) as f32 * spacing.between_columns.value(config);

        Vector2d::new(width, height)
    }

    fn calculate_fullscreen_backdrop_size(&self, config: &RenderingConfig, children: &Vec<View>) -> Vector2d {
        self.calculate_zstack_size(config, children, &Spacing::ZERO)
    }

    fn calculate_fixed_position_size(&self, config: &RenderingConfig, children: &Vec<View>) -> Vector2d {
        self.calculate_zstack_size(config, children, &Spacing::ZERO)
    }
}

pub fn showcase_view() -> View {
    zstack!(
        Spacing::LG,
        Color::BLACK.alpha(0.0),
        zstack!(
            Spacing::LG,
            Color::BLACK,
            hstack!(
                Spacing::LG,
                vstack!(
                    Spacing::ZERO, 
                    text!(TextStyle::Bold, "Hello Bold".to_string()),
                    spacing!(Spacing::MD),
                    text!(TextStyle::Regular, "Hello Regular".to_string()),
                    spacing!(Spacing::LG),
                    hstack!(
                        Spacing::LG,
                        zstack!(
                            Spacing::SM, 
                            Color::YELLOW,
                            texture!(
                                SPRITE_SHEET_INVENTORY, 
                                Rect::new(TILE_SIZE, 0.0, TILE_SIZE, TILE_SIZE), 
                                Vector2d::new(5.0 * TILE_SIZE, 5.0 * TILE_SIZE)
                            )
                        ),
                        texture!(
                            SPRITE_SHEET_INVENTORY, 
                            Rect::new(2.0 * TILE_SIZE, 0.0, TILE_SIZE, TILE_SIZE), 
                            Vector2d::new(10.0 * TILE_SIZE, 10.0 * TILE_SIZE)
                        )
                    )
                ),
                vstack!(
                    Spacing::MD,
                    vstack!(
                        Spacing::SM,
                        text!(TextStyle::Regular, "4 columns".to_string()),
                        vgrid!(
                            4, 
                            GridSpacing::new(Spacing::MD, Spacing::SM),
                            text!(TextStyle::Regular, "0".to_string()),
                            text!(TextStyle::Regular, "1".to_string()),
                            text!(TextStyle::Regular, "2".to_string()),
                            text!(TextStyle::Regular, "3".to_string()),
                            text!(TextStyle::Regular, "4".to_string()),
                            text!(TextStyle::Regular, "5".to_string()),
                            text!(TextStyle::Regular, "6".to_string())
                        )
                    ),
                    vstack!(
                        Spacing::SM,
                        text!(TextStyle::Regular, "4 rows".to_string()),
                        hgrid!(
                            4, 
                            GridSpacing::new(Spacing::SM, Spacing::MD),
                            text!(TextStyle::Regular, "0".to_string()),
                            text!(TextStyle::Regular, "1".to_string()),
                            text!(TextStyle::Regular, "2".to_string()),
                            text!(TextStyle::Regular, "3".to_string()),
                            text!(TextStyle::Regular, "4".to_string()),
                            text!(TextStyle::Regular, "5".to_string()),
                            text!(TextStyle::Regular, "6".to_string())
                        )
                    )
                )
            )
        )
    )
}