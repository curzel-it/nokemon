#[macro_export]
macro_rules! impl_entity {
    ($struct_name:ident) => {
        impl crate::game_engine::entity::Entity for $struct_name {
            fn id(&self) -> u32 {
                self.base.id
            }
            
            fn parent_id(&self) -> u32 {
                self.base.parent_id
            }
            
            fn species(&self) -> &crate::species::species_model::Species {
                &self.base.species
            }
            
            fn frame(&self) -> raylib::math::Rectangle {
                self.base.frame
            }
            
            fn set_frame(&mut self, value: raylib::math::Rectangle) {
                self.base.frame = value;
            }
            
            fn center_in(&mut self, value: &raylib::math::Rectangle) {
                self.base.center_in(value);
            }
            
            fn center_at(&mut self, value: &raylib::math::Vector2) {
                self.base.center_at(value);
            }
            
            fn direction(&self) -> raylib::math::Vector2 {
                self.base.direction
            }
            
            fn set_direction(&mut self, value: raylib::math::Vector2) {
                self.base.direction = value;
            }
            
            fn speed(&self) -> f32 {
                self.base.speed
            }
            
            fn set_speed(&mut self, speed: f32) {
                self.base.speed = speed;
            }
            
            fn reset_speed(&mut self) {
                self.base.reset_speed();
            }
            
            fn hp(&self) -> f32 {
                self.base.hp
            }
            
            fn inc_hp(&mut self, value: f32) {
                self.base.hp += value;
            }
            
            fn current_sprite_frame(&self) -> &str {
                self.base.current_sprite.current_frame()
            }
            
            fn current_animation(&self) -> &str {
                self.base.current_sprite.animation_name.as_str()
            }
            
            fn set_animation(&mut self, animation_name: &str) -> u32 {
                self.base.set_animation(animation_name)
            }

            fn creation_time(&self) -> f32 {
                self.base.creation_time
            }

            fn set_creation_time(&mut self, value: f32) {
                self.base.creation_time = value;
            }
        }
    };
}
