use raylib::prelude::*;

const FONT: &str = "../fonts/joystix monospace.otf";

pub struct DesignSystemConfig
let custom_font = rl.load_font(&thread, FONT).unwrap();

pub fn draw_text(d: &mut RaylibDrawHandle, text: &str) {
    d.draw_text_ex(FONT, "Hello, Raylib with Rust!", position, font_size, font_spacing, Color::BLACK);
}
