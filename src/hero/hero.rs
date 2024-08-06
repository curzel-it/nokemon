use crate::{game_engine::entity_body::EntityBody, impl_embodied_entity};

#[derive(Debug)]
pub struct Hero {
    pub body: EntityBody
}

impl_embodied_entity!(Hero);
