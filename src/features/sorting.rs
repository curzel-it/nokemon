use std::cmp::Ordering;

use raylib::math::Vector2;

use crate::entities::entity::Entity;

pub fn sort_by_distance(origin: Vector2, entities: &mut Vec<&Entity>) {
    entities.sort_by(|a, b| {
        let center_a = Vector2::new(
            a.frame.x + a.frame.width / 2.0, 
            a.frame.y + a.frame.height / 2.0
        );
        let center_b = Vector2::new(
            b.frame.x + b.frame.width / 2.0, 
            b.frame.y + b.frame.height / 2.0
        );
        let dist_a = origin.distance_to(center_a);
        let dist_b = origin.distance_to(center_b);

        dist_a.partial_cmp(&dist_b).unwrap_or(Ordering::Equal)
    });
}