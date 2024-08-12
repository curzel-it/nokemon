use crate::{constants::{BASE_ENTITY_SPEED, COLLISION_BOUNCE_FIX, COLLISION_RIGIDITY_THRESHOLD, SCALE}, game_engine::{collision_detection::Collision, entity::Entity, world::World}};

pub fn move_linearly(entity: &mut dyn Entity, world: &World, time_since_last_update: f32) { 
    if entity.body().is_rigid {
        if let Some(all_collisions) = &world.collisions.get(&entity.id()) {
            let valid_collisions: Vec<&Collision> = all_collisions
                .iter()
                .filter(|c| {
                    c.other_was_rigid && (c.area.width > COLLISION_RIGIDITY_THRESHOLD && c.area.height > COLLISION_RIGIDITY_THRESHOLD)
                })
                .collect();
            
            if !valid_collisions.is_empty() {
                move_away_from_collisions(entity, &valid_collisions);
            }
        }
    }
    
    just_move(entity, time_since_last_update);
}

fn move_away_from_collisions(entity: &mut dyn Entity, collisions: &Vec<&Collision>) {
    let direction = entity.body().direction;
    let frame = entity.body().frame;
    let entity_center_x = frame.x + frame.width / 2.0;
    let entity_center_y = frame.y + frame.height / 2.0;

    if direction.x >= 0.0 {
        entity.body_mut().frame.x = collisions.iter().filter_map(|c| {
            if c.area.x > entity_center_x {
                Some(c.area.x - frame.width - COLLISION_BOUNCE_FIX)
            } else {
                None
            }
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(frame.x);
    }
    if direction.x < 0.0 {
        entity.body_mut().frame.x = collisions.iter().filter_map(|c| {
            if c.area.x + c.area.width < entity_center_x {
                Some(c.area.x + c.area.width + COLLISION_BOUNCE_FIX)
            } else {
                None
            }
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(frame.x);
    }
    if direction.y > 0.0 {
        entity.body_mut().frame.y = collisions.iter().filter_map(|c| {
            if c.area.y > entity_center_y {
                Some(c.area.y - frame.height - COLLISION_BOUNCE_FIX)
            } else {
                None
            }
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(frame.y);
    }
    if direction.y < 0.0 {
        entity.body_mut().frame.y = collisions.iter().filter_map(|c| {
            if c.area.y + c.area.height < entity_center_y {
                Some(c.area.y + c.area.height + COLLISION_BOUNCE_FIX)
            } else {
                None
            }
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(frame.y);
    }
}

fn just_move(entity: &mut dyn Entity, time_since_last_update: f32) {
    let frame = entity.body().frame;
    let offset = entity.body().direction * entity.body().current_speed * time_since_last_update * SCALE * BASE_ENTITY_SPEED;
    let expected_x = frame.x + offset.x;
    let expected_y = frame.y + offset.y;
    entity.place_at(expected_x, expected_y);
}

#[cfg(test)]
mod tests {
    use raylib::math::{Rectangle, Vector2};

    use crate::{constants::{BASE_ENTITY_SPEED, COLLISION_BOUNCE_FIX, RECT_ORIGIN_SQUARE_100, SCALE}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, simple_entity::SimpleEntity, world::World}};
    
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

    #[test]
    fn can_not_move_in_direction_of_collisions_with_rigid_body() {
        let mut world = World::test();
        world.camera_viewport = RECT_ORIGIN_SQUARE_100;
        
        let mut body1 = world.entity_factory.build("red");
        body1.id = 1001;
        body1.frame = Rectangle::new(0.0, 0.0, 10.0, 10.0);
        body1.direction = Vector2::new(1.0, 0.0);  
        body1.requires_collision_detection = true;
        body1.current_speed = 1.0;               
        body1.is_ally = true; 
        body1.is_rigid = true;
        let entity1 = SimpleEntity::new(body1);
        world.add_entity(Box::new(entity1));
        
        let mut body2 = world.entity_factory.build("red");
        body2.id = 1002;
        body2.frame = Rectangle::new(9.0, 0.0, 10.0, 10.0);
        body2.direction = Vector2::zero();
        body2.current_speed = 1.0;
        body2.is_ally = false;
        body2.is_rigid = true;
        let entity2 = SimpleEntity::new(body2);
        world.add_entity(Box::new(entity2));
        
        world.update(1.0);

        let entities = world.entities.borrow();
        let updated_frame = entities.get(&1001).unwrap().body().frame;
        
        assert_eq!(updated_frame.x, -COLLISION_BOUNCE_FIX);
        assert_eq!(updated_frame.y, 0.0);
    }
}