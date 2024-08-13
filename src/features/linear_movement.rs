use crate::{constants::{BASE_ENTITY_SPEED, SCALE}, game_engine::{collision_detection::Collision, entity::Entity, world::World}};

pub fn move_linearly(entity: &mut dyn Entity, world: &World, time_since_last_update: f32) { 
    let no_collisions: Vec<Collision> = vec![];
    let collisions = world.collisions.get(&entity.id()).unwrap_or(&no_collisions);
    let frame = entity.body().frame;
    let offset = entity.body().direction * entity.body().current_speed * time_since_last_update * SCALE * BASE_ENTITY_SPEED;
    let expected_x = frame.x + offset.x;
    let expected_y = frame.y + offset.y;

    if entity.body().is_rigid {
        if has_blocking_collisions(entity, collisions) {
            return
        }
    }

    entity.place_at(expected_x, expected_y);
}

fn has_blocking_collisions(entity: &dyn Entity, collisions: &Vec<Collision>) -> bool {
    !blocking_collisions(entity, collisions).is_empty()
}

fn blocking_collisions(entity: &dyn Entity, collisions: &Vec<Collision>) -> Vec<Collision> {
    let threshold = 20.0;
    let entity_center_x = entity.body().frame.x + entity.body().frame.width / 2.0;
    let entity_center_y = entity.body().frame.y + entity.body().frame.height / 2.0;
    let direction = entity.body().direction;

    if direction.x > 0.0 {
        return collisions.iter().filter(|collision| {
            collision.center_x > entity_center_x && collision.overlapping_area.height > threshold
        })
        .map(|c| c.clone())
        .collect();
    }
    if direction.x < 0.0 {
        return collisions.iter().filter(|collision| {
            collision.center_x < entity_center_x && collision.overlapping_area.height > threshold
        })
        .map(|c| c.clone())
        .collect();
    }
    if direction.y > 0.0 {
        return collisions.iter().filter(|collision| {
            collision.center_y > entity_center_y && collision.overlapping_area.width > threshold
        })
        .map(|c| c.clone())
        .collect();
    }
    if direction.y < 0.0 {
        return collisions.iter().filter(|collision| {
            collision.center_y < entity_center_y && collision.overlapping_area.width > threshold
        })
        .map(|c| c.clone())
        .collect();
    }
    vec![]
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