use crate::{
    entity::{door::Door, entities::Components, entity_id::Entity, player::PlayerData},
    game_data::GameData,
    physics::collision::check_collision_circles,
};

use super::traits::SphereCollider;

pub fn handle_door_collisions(data: &GameData, entities: &Vec<Entity>, components: &Components) {
    let players = entities
        .iter()
        .filter(|e| {
            components.player_data.contains_key(e)
                && components.positions.contains_key(e)
                && components.colliders.contains_key(e)
        })
        .collect::<Vec<&Entity>>();

    let doors = entities
        .iter()
        .filter(|e| components.positions.contains_key(e) && components.doors.contains_key(e))
        .collect::<Vec<&Entity>>();

    for player_e in &players {
        let position = components.positions.get(player_e).unwrap();
        let player_collider = components.colliders.get(player_e).unwrap();

        for door_e in &doors {
            let collider = components.colliders.get(door_e).unwrap();
            let coll_position = components.positions.get(door_e).unwrap();
            if let Some(_) = check_collision_circles(
                data,
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
