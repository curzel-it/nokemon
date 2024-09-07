use std::any::Any;

use serde::{Deserialize, Serialize};
use crate::{constants::INFINITE_LIFESPAN, dialogues::dialogues::{Dialogue, EntityDialogues}, features::{animated_sprite::AnimatedSprite, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, storage::get_value_for_key, world::World}, impl_embodied_entity, impl_humanoid_sprite_update, lang::localizable::LocalizableText, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcType {
    OldMan,
    OldWoman,
}

pub type NpcId = u32;

impl NpcType {
    fn build_sprite(&self) -> AnimatedSprite {
        let index = match self {
            NpcType::OldMan => 1,
            NpcType::OldWoman => 2,
        };
        AnimatedSprite::new_humanoid(index)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Npc {
    body: EntityBody,
    npc_type: NpcType,
    name: String,
    sprite: AnimatedSprite,
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
                dialogues: EntityDialogues::empty(),
            },
            npc_type,
            name: "npc.default_name".localized(),
            sprite: npc_type.build_sprite(),
        }
    }

    fn next_dialogue(&self) -> Option<Dialogue> {
        for option in &self.body.dialogues.options {
            if let Some(value) = get_value_for_key(&option.key) {
                if value == option.expected_value {
                    return Some(option.dialogue.clone())
                }
            }
        }
        None
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
                            self.body.id, self.name.clone(), self.next_dialogue()
                        )
                    )
                ];  
            } else if let Some(dialogue) = self.next_dialogue() {
                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowDialogue(
                            self.body.id, self.name.clone(), dialogue,
                        )
                    )
                ];
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
