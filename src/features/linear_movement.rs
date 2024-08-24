use crate::{constants::{BASE_ENTITY_SPEED, COLLISION_THRESHOLD}, game_engine::{collision_detection::Collision, entity::Entity, world::World}};

pub fn move_linearly(entity: &mut dyn Entity, world: &World, time_since_last_update: f32) { 
    let no_collisions: Vec<Collision> = vec![];
    let collisions = world.collisions.get(&entity.id()).unwrap_or(&no_collisions);
    let frame = entity.body().frame;
    
    let offset = entity.body().direction
        .scaled(entity.body().current_speed)
        .scaled(time_since_last_update)
        .scaled(BASE_ENTITY_SPEED);
    
    let expected_x = frame.x + offset.x;
    let expected_y = frame.y + offset.y;

    if has_blocking_collisions(entity, collisions) {
        return
    }

    entity.place_at(expected_x, expected_y);
}

fn has_blocking_collisions(entity: &dyn Entity, collisions: &Vec<Collision>) -> bool {
    if entity.body().is_rigid && entity.body().requires_collision_detection {
        let rigid_collisions: Vec<&Collision> = collisions.iter().filter(|c| c.other_was_rigid).collect();
        return has_blocking_rigid_collisions(entity, &rigid_collisions);        
    }
    false
}

fn has_blocking_rigid_collisions(entity: &dyn Entity, collisions: &Vec<&Collision>) -> bool {
    let entity_center_x = entity.body().frame.x + entity.body().frame.w / 2.0;
    let entity_center_y = entity.body().frame.y + entity.body().frame.h / 2.0;
    let direction = entity.body().direction;

    if direction.x > 0.0 {
        return collisions.iter().any(|collision| {
            collision.center_x > entity_center_x && collision.overlapping_area.h > COLLISION_THRESHOLD
        });
    }
    if direction.x < 0.0 {
        return collisions.iter().any(|collision| {
            collision.center_x < entity_center_x && collision.overlapping_area.h > COLLISION_THRESHOLD
        });
    }
    if direction.y > 0.0 {
        return collisions.iter().any(|collision| {
            collision.center_y > entity_center_y && collision.overlapping_area.w > COLLISION_THRESHOLD
        });
    }
    if direction.y < 0.0 {
        return collisions.iter().any(|collision| {
            collision.center_y < entity_center_y && collision.overlapping_area.w > COLLISION_THRESHOLD
        });
    }
    false
}

#[cfg(test)]
mod tests {
        use crate::{constants::{BASE_ENTITY_SPEED, RECT_ORIGIN_SQUARE_100}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, simple_entity::SimpleEntity, world::World}, worlds::constants::WORLD_DEMO_WORLD, utils::vector::Vector2d};
    
    #[test]
    fn can_move_on_update() {
        let world = World::new(WORLD_DEMO_WORLD);
        
        let mut body = EntityBody::test();
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;        
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2d::new(1.0, 0.0);  
        entity.update(&world, 1.0);

        assert_eq!(entity.body().frame.x, BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, 0.0);
    }

    #[test]
    fn can_move_outside_of_bounds() {
        let world = World::new(WORLD_DEMO_WORLD);
        
        let mut body = EntityBody::test();
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2d::new(-1.0, 0.0);  
        entity.update(&world, 1.0);

        assert_eq!(entity.body().frame.x, -BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, 0.0);
    }
}