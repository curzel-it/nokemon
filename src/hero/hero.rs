use crate::{game_engine::entity::BaseEntity, impl_entity};

#[derive(Debug)]
pub struct Hero {
    pub base: BaseEntity
}

impl_entity!(Hero);
