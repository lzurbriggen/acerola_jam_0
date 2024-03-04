use crate::{entity::entities::Ecs, physics::collision::check_collision_circles};

pub fn handle_door_collisions(ecs: &Ecs) {
    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e)
            && comps.positions.contains_key(e)
            && comps.colliders.contains_key(e)
    });

    let doors = ecs.check_components(|e, comps| {
        comps.positions.contains_key(e) && comps.doors.contains_key(e)
    });

    for player_e in &players {
        let position = ecs.components.positions.get(player_e).unwrap();
        let player_collider = ecs.components.colliders.get(player_e).unwrap();

        for door_e in &doors {
            let collider = ecs.components.colliders.get(door_e).unwrap();
            let coll_position = ecs.components.positions.get(door_e).unwrap();
            if let Some(_) = check_collision_circles(
                *position,
                player_collider.radius,
                *coll_position,
                collider.radius,
            ) {
                println!("{:?}", "DOOR COLLISION");
            }
        }
    }
}
