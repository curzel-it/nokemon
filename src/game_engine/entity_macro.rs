#[macro_export]
macro_rules! impl_embodied_entity {
    ($struct_name:ident) => {
        impl crate::game_engine::entity_body::EmbodiedEntity for $struct_name {
            fn id(&self) -> u32 {
                self.body.id
            }
            
            fn parent_id(&self) -> u32 {
                self.body.parent_id
            }
            
            fn species(&self) -> &crate::species::species_model::Species {
                &self.body.species
            }
            
            fn frame(&self) -> raylib::math::Rectangle {
                self.body.frame
            }
            
            fn set_frame(&mut self, value: raylib::math::Rectangle) {
                self.body.frame = value;
            }
            
            fn center_in(&mut self, value: &raylib::math::Rectangle) {
                self.body.center_in(value);
            }
            
            fn center_at(&mut self, value: &raylib::math::Vector2) {
                self.body.center_at(value);
            }

            fn place_at(&mut self, x: f32, y: f32) {
                self.body.frame.x = x;
                self.body.frame.y = y;
            }
            
            fn direction(&self) -> raylib::math::Vector2 {
                self.body.direction
            }
            
            fn set_direction(&mut self, value: raylib::math::Vector2) {
                self.body.direction = value;
            }
            
            fn speed(&self) -> f32 {
                self.body.speed
            }
            
            fn set_speed(&mut self, speed: f32) {
                self.body.speed = speed;
            }
            
            fn reset_speed(&mut self) {
                self.body.reset_speed();
            }
            
            fn hp(&self) -> f32 {
                self.body.hp
            }
            
            fn inc_hp(&mut self, value: f32) {
                self.body.hp += value;
            }
            
            fn current_sprite_frame(&self) -> &str {
                self.body.current_sprite.current_frame()
            }
            
            fn current_animation(&self) -> &str {
                self.body.current_sprite.animation_name.as_str()
            }
            
            fn set_animation(&mut self, animation_name: &str) -> u32 {
                self.body.set_animation(animation_name)
            }

            fn creation_time(&self) -> f32 {
                self.body.creation_time
            }

            fn set_creation_time(&mut self, value: f32) {
                self.body.creation_time = value;
            }
        }
    };
}
