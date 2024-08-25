#[macro_export]
macro_rules! impl_embodied_entity {
    ($struct_name:ident) => {
        impl $crate::game_engine::entity_body::EmbodiedEntity for $struct_name {
            fn id(&self) -> uuid::Uuid {
                self.body.id
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
        }
    };
}
