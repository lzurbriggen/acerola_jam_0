use crate::entity::entities::Ecs;
use macroquad::prelude::*;

pub fn flash_on_damage(ecs: &mut Ecs) {
    let damageables = ecs.check_components(|e, comps| {
        comps.damageables.contains_key(e) && comps.materials.contains_key(e)
    });

    for damageable_e in &damageables {
        let damageable = ecs.components.damageables.get_mut(damageable_e).unwrap();
        let material = ecs.components.materials.get_mut(damageable_e).unwrap();

        if let Some(invulnerable_timer) = &mut damageable.invulnerable_timer {
            invulnerable_timer.update();
        }

        if let Some(hit_fx_timer) = &mut damageable.hit_fx_timer {
            // TODO: move to separate component
            hit_fx_timer.update();
            if !hit_fx_timer.completed() {
                let intensity = hit_fx_timer.progress() * 10. + 1.;
                let mut color = WHITE;
                color.r = intensity;
                color.g = intensity;
                color.b = intensity;
                color.a = (0.5 - hit_fx_timer.progress() % 0.5) * 2.;
                material.set_uniform("color", color);
            } else {
                material.set_uniform("color", WHITE);
            }
        }
    }
}
