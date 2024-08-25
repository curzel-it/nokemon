use super::vector::Vector2d;

pub fn directions_based_direction_vector_4d(up: bool, right: bool, down: bool, left: bool) -> Option<Vector2d> {
    if up { return Some(Vector2d::new(0.0, -1.0)); }
    if right { return Some(Vector2d::new(1.0, 0.0)); }
    if down { return Some(Vector2d::new(0.0, 1.0)); }
    if left { return Some(Vector2d::new(-1.0, -0.0)); }
    None
}