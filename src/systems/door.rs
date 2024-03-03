use crate::{
    entity::entities::Entities, game_data::GameData, physics::collision::check_collision_circles,
};

pub fn handle_door_collisions(data: &GameData, entities: &mut Entities) {
    let player = &entities.player;
    for door in &entities.doors {
        if let Some(_) = check_collision_circles(
            data,
            player.position,
            player.collider_radius,
            door.position,
            door.radius,
        ) {
            println!("{:?}", door.position);
        }
    }
}
