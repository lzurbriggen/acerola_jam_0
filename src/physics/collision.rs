use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    entity::entity_id::Entity, game_data::GameData, map::map::Map,
    systems::collision::CircleCollider,
};

#[derive(Debug, PartialEq)]
pub struct Collision {
    pub point: Vec2,
    pub overlap: f32,
    pub normal: Vec2,
}

pub fn check_collision_circles(
    pos1: Vec2,
    radius1: f32,
    pos2: Vec2,
    radius2: f32,
) -> Option<Collision> {
    let combined_radii = radius1 + radius2;
    let diff = pos1 - pos2;
    let overlap = diff.length() - combined_radii;

    if overlap < 0. {
        let normal = diff.normalize();
        let point = normal * radius2;
        return Some(Collision {
            point,
            normal,
            overlap,
        });
    }

    None
}

pub fn resolve_circle_collision(
    source_entity: Entity,
    pos: Vec2,
    collider: &CircleCollider,
    colliders: &Vec<(Entity, Vec2, &CircleCollider)>,
) -> (Vec2, HashMap<(Entity, Entity), Collision>) {
    let collider = colliders
        .iter()
        .find(|(e, _, _)| source_entity == *e)
        .unwrap();

    let mut desired_pos = pos;
    let mut collisions = HashMap::new();
    for _ in 0..2 {
        let mut is_colliding = false;
        for (coll_e, other_pos, other_coll) in colliders {
            if *coll_e == source_entity {
                break;
            }
            if let Some(collision) = check_collision_circles(
                desired_pos,
                collider.2.radius,
                *other_pos,
                other_coll.radius,
            ) {
                is_colliding = true;
                if !other_coll.trigger && !collider.2.trigger {
                    desired_pos = desired_pos - collision.normal * (collision.overlap + 0.01);
                }
                collisions.insert((source_entity, *coll_e), collision);
            }
        }
        if !is_colliding {
            break;
        }
    }
    (desired_pos, collisions)
}

pub fn check_collision_aabb_circle(
    data: &GameData,
    rect: &Rect,
    circle_pos: Vec2,
    circle_radius: f32,
) -> Option<Vec2> {
    let center = rect.center();
    let diff = circle_pos - center;
    let clamped_distance = diff.clamp(
        vec2(-rect.w / 2., -rect.h / 2.),
        vec2(rect.w / 2., rect.h / 2.),
    );
    let closest_point = center + clamped_distance;

    if data.debug_collisions {
        draw_circle_lines(circle_pos.x, circle_pos.y, circle_radius, 1., BLUE);
        draw_circle(closest_point.x, closest_point.y, 1., GREEN);
    }

    let is_colliding =
        (closest_point - circle_pos).length_squared() < circle_radius * circle_radius;
    if data.debug_collisions && is_colliding {
        draw_circle(closest_point.x, closest_point.y, 1., RED);
    }

    if is_colliding {
        if data.debug_collisions {
            draw_circle(closest_point.x, closest_point.y, 1., RED);
        }
        return Some(closest_point);
    }

    None
}

pub fn resolve_map_collision(data: &GameData, map: &Map, pos: Vec2, radius: f32) -> Vec2 {
    let mut desired_pos = pos;
    let tile_position = desired_pos / 8.;
    let tile_position = (tile_position.x as i32, tile_position.y as i32);
    let area_size = 1;
    let area = (
        (tile_position.0 - area_size)..=tile_position.0 + area_size,
        (tile_position.1 - area_size)..=tile_position.1 + area_size,
    );
    for _ in 0..2 {
        let mut is_colliding = false;
        for (x, y) in &map.map_collision {
            if !(area.0.contains(&(*x as i32)) && area.1.contains(&(*y as i32))) {
                continue;
            }
            let rect = Rect::new(*x as f32 * 8., *y as f32 * 8., 8., 8.);
            if let Some(closest_point) =
                check_collision_aabb_circle(&data, &rect, desired_pos, radius)
            {
                is_colliding = true;
                let diff_to_closest_point = closest_point - desired_pos;
                let overlap = radius - diff_to_closest_point.length();

                desired_pos = desired_pos - diff_to_closest_point.normalize() * (overlap + 0.01);
            }
        }
        if !is_colliding {
            break;
        }
    }
    desired_pos
}
