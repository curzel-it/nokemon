use crate::{constants::{BASE_ENTITY_SPEED, SCALE}, game_engine::{entity::Entity, world::World}};

pub fn move_linearly(entity: &mut dyn Entity, _: &World, time_since_last_update: f32) { 
    let frame = entity.body().frame;
    let offset = entity.body().direction * entity.body().current_speed * time_since_last_update * SCALE * BASE_ENTITY_SPEED;
    let expected_x = frame.x + offset.x;
    let expected_y = frame.y + offset.y;
    entity.place_at(expected_x, expected_y);
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::{BASE_ENTITY_SPEED, RECT_ORIGIN_SQUARE_100, SCALE}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, simple_entity::SimpleEntity, world::World}};
    
    #[test]
    fn can_move_on_update() {
        let world = World::test();
        
        let mut body = world.entity_factory.build("red");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;        
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2::new(1.0, 0.0);  
        entity.update(&world, 1.0);

        assert_eq!(entity.body().frame.x, SCALE * BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, 0.0);
    }

    #[test]
    fn can_move_outside_of_bounds() {
        let world = World::test();
        
        let mut body = world.entity_factory.build("red");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2::new(-1.0, 0.0);  
        entity.update(&world, 1.0);

        assert_eq!(entity.body().frame.x, -SCALE * BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, 0.0);
    }
}