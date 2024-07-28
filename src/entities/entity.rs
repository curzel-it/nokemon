use std::fmt::{self, Debug};

use raylib::math::{Rectangle, Vector2};

use crate::constants::{ANIMATIONS_FPS, SPRITE_NAME_MOVEMENT};
use crate::game::rendered_item::RenderedItem;
use crate::sprites::sprite::Sprite;
use crate::sprites::sprite_set::SpriteSet;

pub struct Entity {
    pub id: u32,
    pub frame: Rectangle,
    pub direction: Vector2,
    pub speed: f32,
    pub species: String,
    sprite_set: SpriteSet,
    current_sprite: Sprite,
    pub is_enemy: bool,
}

pub trait EntityCapability {
    fn update(&self, time_since_last_update: f32, entity: &mut Entity);
}

impl Entity {
    pub fn new(
        id: u32,
        speed: f32,
        species: String,
        sprite_set: SpriteSet,
        frame: Rectangle,
    ) -> Self {
        let mut entity = Self {
            id,
            frame,
            direction: Vector2::new(1.0, 0.0),
            speed: speed,
            species,
            sprite_set,
            current_sprite: Sprite::new("".to_owned(), Vec::new(), 1.0),
            is_enemy: false,
        };
        entity.change_sprite(SPRITE_NAME_MOVEMENT);
        entity
    }

    pub fn current_sprite_frame(&self) -> String {
        self.current_sprite.current_frame().to_string()
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.current_sprite.update(time_since_last_update);
        self.linear_movement(time_since_last_update);
    }

    fn linear_movement(&mut self, time_since_last_update: f32) {
        let offset = self.direction * self.speed * time_since_last_update;
        let mut updated_frame = self.frame;
        updated_frame.x += offset.x;
        updated_frame.y += offset.y;
        self.frame = updated_frame;
    }

    fn change_sprite(&mut self, animation_name: &str) -> u32 {
        if self.current_sprite.animation_name != animation_name {
            self.current_sprite = self.sprite_set.sprite(&animation_name, ANIMATIONS_FPS);
        }
        ((self.current_sprite.number_of_frames() as f32) / ANIMATIONS_FPS) as u32
    }

    pub fn render(&self) -> RenderedItem {
        return RenderedItem::new(
            self.id, 
            self.current_sprite_frame(), 
            self.frame, 
            self.direction.x < 0.0, 
            0.0
        );
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity")
            .field("id", &self.id)
            .field("sprite", &self.current_sprite_frame())
            .field("speed", &self.speed)
            .field("dx", &self.direction.x)
            .field("dy", &self.direction.y)        
            .field("x", &self.frame.x)
            .field("y", &self.frame.y)
            .field("width", &self.frame.width)
            .field("height", &self.frame.height)
            .field("is_enemy", &self.is_enemy)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, entities::factory::EntityFactory};

    #[test]
    fn can_move_entity_horizontally_on_update() {
        let entity_factory = EntityFactory::test();

        let mut entity = entity_factory.build("ape");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.direction = Vector2::new(1.0, 0.0);
                
        entity.update(1.0);
        assert_eq!(entity.frame.x, 30.0);
    }

    #[test]
    fn can_move_entity_vertically_on_update() {
        let entity_factory = EntityFactory::test();

        let mut entity = entity_factory.build("ape");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.direction = Vector2::new(0.0, 1.0);

        entity.update(1.0);
        assert_eq!(entity.frame.y, 30.0);
    }
}