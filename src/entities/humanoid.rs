#[macro_export]
macro_rules! impl_humanoid_sprite_update {
    ($struct_name:ident) => {
        impl $struct_name {
            fn update_sprite(&mut self, time_since_last_update: f32) {
                let direction = crate::utils::geometry_utils::Direction::from_vector(self.body.direction);
                let is_moving = self.body.current_speed != 0.0;
        
                self.sprite.row = match (direction, is_moving) {
                    (crate::utils::geometry_utils::Direction::Up, true) => 0.0,
                    (crate::utils::geometry_utils::Direction::Up, false) => 1.0,
                    (crate::utils::geometry_utils::Direction::Right, true) => 2.0,
                    (crate::utils::geometry_utils::Direction::Right, false) => 3.0,
                    (crate::utils::geometry_utils::Direction::Down, true) => 4.0,
                    (crate::utils::geometry_utils::Direction::Down, false) => 5.0,
                    (crate::utils::geometry_utils::Direction::Left, true) => 6.0,
                    (crate::utils::geometry_utils::Direction::Left, false) => 7.0,
                    (crate::utils::geometry_utils::Direction::Unknown, true) => 5.0,
                    (crate::utils::geometry_utils::Direction::Unknown, false) => 5.0
                };
                self.sprite.update(time_since_last_update);
            }
        }
    };
}
