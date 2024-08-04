use raylib::math::Vector2;

pub fn direction_vector(x1: f32, y1: f32, x2: f32, y2: f32) -> Vector2 {
    let dx = x2 - x1;
    let dy = y2 - y1;    
    let length = (dx.powi(2) + dy.powi(2)).sqrt();

    if length != 0.0 {
        return Vector2::new(dx / length, dy / length);
    } else {
        return Vector2 { x: 0.0, y: 0.0 }
    }
}

pub fn dumb_direction_vector(x1: f32, y1: f32, x2: f32, y2: f32) -> Vector2 {
    let up = y2 < y1;
    let right = x2 > x1;
    let down = y2 > y1;
    let left = x2 < x1;
    return directions_based_direction_vector(up, right, down, left).unwrap_or_default();
}

pub fn directions_based_direction_vector(up: bool, right: bool, down: bool, left: bool) -> Option<Vector2> {
    if up && left { return Some(Vector2::new(-0.71, -0.71)); }
    if up && right { return Some(Vector2::new(0.71, -0.71)); }
    if down && left { return Some(Vector2::new(-0.71, 0.71)); }
    if down && right { return Some(Vector2::new(0.71, 0.71)); }
    
    if up { return Some(Vector2::new(0.0, -1.0)); }
    if right { return Some(Vector2::new(1.0, 0.0)); }
    if down { return Some(Vector2::new(0.0, 1.0)); }
    if left { return Some(Vector2::new(-1.0, -0.0)); }

    return None;
}