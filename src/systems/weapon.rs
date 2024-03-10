use macroquad::{
    audio::{self, PlaySoundParams},
    math::{vec2, Vec2},
    texture::Texture2D,
};

use crate::{
    entity::{entities::Ecs, projectile::spawn_bullet, tags::EntityType},
    game_data::GameData,
    items::weapon::Weapon,
};

pub fn update_weapon(ecs: &mut Ecs, data: &mut GameData) {
    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e) && comps.positions.contains_key(e)
    });

    let mut bullet_data = Vec::<(f32, Vec2, Vec2)>::new();

    for player_e in &players {
        let position = ecs.components.positions.get_mut(&player_e).unwrap();
        match &mut data.weapon {
            Weapon::Shooter(ref mut shooter) => {
                shooter.shoot_timer.update();
                if shooter.shoot_timer.just_completed() {
                    bullet_data.push((shooter.damage, *position + vec2(3., 0.), vec2(160., 0.)));
                    bullet_data.push((shooter.damage, *position + vec2(-3., 0.), vec2(-160., 0.)));
                    bullet_data.push((shooter.damage, *position + vec2(0., 3.), vec2(0., 160.)));
                    bullet_data.push((shooter.damage, *position + vec2(0., -3.), vec2(0., -160.)));
                }
            }
        }
    }

    if bullet_data.len() > 0 {
        audio::play_sound(
            &data.audio.shoot,
            PlaySoundParams {
                volume: data.settings.sfx_volume * 0.5,
                ..Default::default()
            },
        );
    }

    for (damage, position, vel) in &bullet_data {
        spawn_bullet(data, ecs, *position, EntityType::Enemy, *damage, *vel);
    }
}
