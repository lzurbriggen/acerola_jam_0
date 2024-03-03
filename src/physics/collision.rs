use macroquad::prelude::*;

use crate::{game_data::GameData, map::map::Map};

pub fn check_collision_circles(
    data: &GameData,
    pos1: Vec2,
    radius1: f32,
    pos2: Vec2,
    radius2: f32,
) -> Option<Vec2> {
    // let center = rect.center();
    // let diff = circle_pos - center;
    // let clamped_distance = diff.clamp(
    //     vec2(-rect.w / 2., -rect.h / 2.),
    //     vec2(rect.w / 2., rect.h / 2.),
    // );
    // let closest_point = center + clamped_distance;

    // if data.debug_collisions {
    //     draw_circle_lines(circle_pos.x, circle_pos.y, circle_radius, 1., BLUE);
    //     draw_circle(closest_point.x, closest_point.y, 1., GREEN);
    // }

    let combined_radii = radius1 + radius2;
    let is_colliding = (pos1 - pos2).length_squared() < combined_radii * combined_radii;
    // if data.debug_collisions && is_colliding {
    //     draw_circle(closest_point.x, closest_point.y, 1., RED);
    // }

    if is_colliding {
        // if data.debug_collisions {
        //     draw_circle(closest_point.x, closest_point.y, 1., RED);
        // }
        return Some(Vec2::ZERO);
    }

    None
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
