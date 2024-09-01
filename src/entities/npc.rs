use std::any::Any;

use serde::{Deserialize, Serialize};
use crate::{constants::{DIALOGUE_ID_NONE, INFINITE_LIFESPAN}, features::{animated_sprite::AnimatedSprite, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, impl_humanoid_sprite_update, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcType {
    OldMan
}

impl NpcType {
    fn build_sprite(&self) -> AnimatedSprite {
        let index = match self {
            NpcType::OldMan => 0,
        };
        AnimatedSprite::new_humanoid(index)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Npc {
    body: EntityBody,
    npc_type: NpcType,
    sprite: AnimatedSprite,

    #[serde(default)]
    dialogue_id: u32,
}

impl Npc {
    pub fn new(npc_type: NpcType) -> Self {
        Self {             
            body: EntityBody {
                id: get_next_id(),
                frame: Rect::new(0, 0, 1, 2),
                offset: Vector2d::zero(),
                direction: Direction::Unknown,
                current_speed: 1.5,
                base_speed: 1.5,
                creation_time: 0.0,
                is_rigid: true,
                z_index: 0,
                lifespan: INFINITE_LIFESPAN,
            },
            npc_type,
            sprite: npc_type.build_sprite(),
            dialogue_id: DIALOGUE_ID_NONE
        }
    }
}

impl_embodied_entity!(Npc);
impl_humanoid_sprite_update!(Npc);

impl Entity for Npc {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        if world.is_hero_around_and_on_collision_with(&self.body.frame) {
            if world.creative_mode {
                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowNpcOptions(
                            self.body.id, self.dialogue_id
                        )
                    )
                ]  
            } else {
                if self.dialogue_id != DIALOGUE_ID_NONE {
                    return vec![
                        WorldStateUpdate::EngineUpdate(
                            EngineStateUpdate::ShowDialogue(
                                self.body.id,
                                0,
                            )
                        )
                    ]
                }
            }             
        }

        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
        vec![]
    }

    fn texture_source_rect(&self) -> Rect {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet(&self) -> u32 {
        self.sprite.sheet_id
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}
