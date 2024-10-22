use crate::zstack;

use super::components::{padding, with_backdrop, with_textured_border, BordersTextures, NonColor, Spacing, View};

pub fn scaffold(backdrop: bool, background_color: NonColor, texture: Option<BordersTextures>, content: View) -> View {
    let mut view = zstack!(Spacing::LG, background_color, content);

    if let Some(texture) = texture {
        view = with_textured_border(texture, view);
    }
    view = padding(Spacing::LG, view);
    
    if backdrop {
        view = with_backdrop(view)
    }
    view
}