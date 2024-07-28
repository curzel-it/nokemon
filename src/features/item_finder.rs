use raylib::math::Vector2;

use crate::game::rendered_item::RenderedItem;

pub fn find_item(point: Vector2, items: &Vec<RenderedItem>) -> Option<&RenderedItem> {
    for item in items {
        if item.frame.check_collision_point_rec(point) {
            return Some(item);
        }
    }
    return None;
}