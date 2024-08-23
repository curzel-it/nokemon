
use crate::{constants::SPRITE_SHEET_TOWER_DART, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, check_bullet_collisions::handle_collisions_for_bullet, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, state_updates::WorldStateUpdate, world::World}, impl_bullet_sprite_update, impl_embodied_entity, utils::{geometry_utils::Insets, rect::Rect}};

#[derive(Debug)]
pub struct TowerDart {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl TowerDart {
    pub fn new(parent: &dyn Entity) -> Self {
        let mut entity = Self {
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: parent.id(),
                frame: Rect::new(0.0, 0.0, 10.0, 10.0),
                collision_insets: Insets::zero(),
                direction: parent.body().direction,
                current_speed: 5.0,
                base_speed: 5.0,
                hp: 100.0,
                dp: 60.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: false,
                z_index: 0,
                is_ally: parent.body().is_ally,
                lifespan: 10.0,
            },
            sprite: AnimatedSprite::new(
                SPRITE_SHEET_TOWER_DART, 
                3, 
                10, 
                10
            )
        };
        entity.center_in(&parent.body().frame);
        entity
    }
}

impl_embodied_entity!(TowerDart);
impl_bullet_sprite_update!(TowerDart);

impl Entity for TowerDart {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut handle_collisions_for_bullet(self, world));
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }

    fn texture_source_rect(&self) -> Rect {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet(&self) -> u32 {
        self.sprite.sheet_id
    }
}