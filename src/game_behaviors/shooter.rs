use crate::game_engine::{game::Game, game_behavior::GameBehavior};

#[derive(Debug)]
pub struct Shooter;

impl Shooter {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameBehavior for Shooter {
    fn update(&self, entity_id: &u32, game: &mut Game, time_since_last_update: f32) {
        let entity = game.entities.get_mut(entity_id).unwrap();
        if !entity.is_shooter {
            return; 
        }

        entity.time_to_next_shot -= time_since_last_update;
        
        if entity.time_to_next_shot <= 0.0 {
            entity.time_to_next_shot = entity.time_between_shots - entity.time_to_next_shot;
            /*let bullet = self.build_bullet(game);
            game.add_entity(bullet);
        }
    }
}

impl Shooter {
    fn build_bullet(&self, game: &Game) -> Entity {
        Entity::new()
    }
}
*/
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_shoot_stuff() {
        // ...
    }
}