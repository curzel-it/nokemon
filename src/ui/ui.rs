use raylib::prelude::*;

// Configuration structures
pub struct UiConfig {
    pub font: Font,
    pub font_bold: Font,
}

// Enum for TextStyle
pub enum TextStyle {
    Bold,
    Regular,
}

// Enum for UiView types
pub enum UiView {
    Box { padding: f32, background_color: Color, children: Vec<UiView> },
    Column { spacing: f32, children: Vec<UiView> },
    Row { spacing: f32, children: Vec<UiView> },
    Text { style: TextStyle, text: String },
}

impl UiConfig {
    fn font(&self, style: &TextStyle) -> &Font {
        match style {
            TextStyle::Bold => &self.font_bold,
            TextStyle::Regular => &self.font,
        }
    }
}

impl UiView {
    // Main render method
    pub fn render(&self, d: &mut RaylibDrawHandle, config: &UiConfig, position: Vector2) {
        match self {
            UiView::Box { padding, background_color, children } => {
                self.render_box(d, config, position, children, *padding, *background_color);
            }
            UiView::Column { spacing, children } => {
                self.render_column(d, config, position, children, *spacing);
            }
            UiView::Row { spacing, children } => {
                self.render_row(d, config, position, children, *spacing);
            }
            UiView::Text { style, text } => {
                self.render_text(d, config, position, style, text);
            }
        }
    }

    // Box rendering logic
    fn render_box(
        &self,
        d: &mut RaylibDrawHandle,
        config: &UiConfig,
        position: Vector2,
        children: &Vec<UiView>,
        padding: f32,
        background_color: Color,
    ) {
        // Render the box background
        let size = self.calculate_size(d, config);
        d.draw_rectangle_v(position, size, background_color);

        // Render each child in the box, positioning relative to the top-left corner
        let mut child_position = position + Vector2::new(padding, padding);
        for child in children {
            child.render(d, config, child_position);
            child_position.y += child.calculate_size(d, config).y + padding;
        }
    }

    // Column rendering logic
    fn render_column(
        &self,
        d: &mut RaylibDrawHandle,
        config: &UiConfig,
        position: Vector2,
        children: &Vec<UiView>,
        spacing: f32,
    ) {
        let mut child_position = position;

        for child in children {
            child.render(d, config, child_position);
            child_position.y += child.calculate_size(d, config).y + spacing;
        }
    }

    // Row rendering logic
    fn render_row(
        &self,
        d: &mut RaylibDrawHandle,
        config: &UiConfig,
        position: Vector2,
        children: &Vec<UiView>,
        spacing: f32,
    ) {
        let mut child_position = position;

        for child in children {
            child.render(d, config, child_position);
            child_position.x += child.calculate_size(d, config).x + spacing;
        }
    }

    // Text rendering logic
    fn render_text(
        &self,
        d: &mut RaylibDrawHandle,
        config: &UiConfig,
        position: Vector2,
        style: &TextStyle,
        text: &String,
    ) {
        let font = config.font(style);
        d.draw_text_ex(font, text, position, 20.0, 1.0, Color::WHITE);
    }

    // Main calculate_size method
    fn calculate_size(&self, d: &RaylibDrawHandle, config: &UiConfig) -> Vector2 {
        match self {
            UiView::Box { padding, background_color: _, children } => {
                self.calculate_box_size(d, config, children, *padding)
            }
            UiView::Column { spacing, children } => {
                self.calculate_column_size(d, config, children, *spacing)
            }
            UiView::Row { spacing, children } => {
                self.calculate_row_size(d, config, children, *spacing)
            }
            UiView::Text { style, text } => {
                self.calculate_text_size(d, config, style, text)
            }
        }
    }

    // Box size calculation logic
    fn calculate_box_size(
        &self,
        d: &RaylibDrawHandle,
        config: &UiConfig,
        children: &Vec<UiView>,
        padding: f32,
    ) -> Vector2 {
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(d, config);
            max_width = max_width.max(size.x + padding * 2.0);
            max_height = max_height.max(size.y + padding * 2.0);
        }

        Vector2::new(max_width, max_height)
    }

    // Column size calculation logic
    fn calculate_column_size(
        &self,
        d: &RaylibDrawHandle,
        config: &UiConfig,
        children: &Vec<UiView>,
        spacing: f32,
    ) -> Vector2 {
        let mut total_height: f32 = 0.0;
        let mut max_width: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(d, config);
            total_height += size.y + spacing;
            max_width = max_width.max(size.x);
        }
        if children.len() > 0 {
            total_height -= spacing;
        }

        Vector2::new(max_width, total_height)
    }

    // Row size calculation logic
    fn calculate_row_size(
        &self,
        d: &RaylibDrawHandle,
        config: &UiConfig,
        children: &Vec<UiView>,
        spacing: f32,
    ) -> Vector2 {
        let mut total_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for child in children {
            let size = child.calculate_size(d, config);
            total_width += size.x + spacing;
            max_height = max_height.max(size.y);
        }
        if children.len() > 0 {
            total_width -= spacing;
        }

        Vector2::new(total_width, max_height)
    }

    // Text size calculation logic
    fn calculate_text_size(
        &self,
        d: &RaylibDrawHandle,
        config: &UiConfig,
        style: &TextStyle,
        text: &String,
    ) -> Vector2 {
        let font = config.font(style);
        let size = font.measure_text(text, 20.0, 1.0);
        Vector2::new(size.x, size.y)
    }
}
