#[macro_export]
macro_rules! impl_embodied_entity {
    ($struct_name:ident) => {
        impl $crate::game_engine::entity_body::EmbodiedEntity for $struct_name {
            fn id(&self) -> u32 {
                self.body.id
            }
            
            fn parent_id(&self) -> u32 {
                self.body.parent_id
            }

            fn body(&self) -> &EntityBody {
                &self.body
            }

            fn body_mut(&mut self) -> &mut EntityBody {
                &mut self.body
            }
            
            fn center_in(&mut self, value: &$crate::utils::rect::Rect) {
                self.body.center_in(value);
            }
            
            fn snap_to_nearest_tile(&mut self) {
                self.body.snap_to_nearest_tile()
            }

            fn props(&self) -> $crate::game_engine::entity::EntityProps {
                self.body.props()
            }

            fn place_at(&mut self, x: f32, y: f32) {
                self.body.frame.x = x;
                self.body.frame.y = y;
            }

            fn collision_frame(&self) -> $crate::utils::rect::Rect {
                self.body.collision_frame()
            }
        }
    };
}
