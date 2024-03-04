use crate::entity::entities::Ecs;

pub fn update_timers(ecs: &mut Ecs) {
    let timers = ecs.check_components(|e, comps| comps.timers.contains_key(e));

    for timer_e in &timers {
        let timer = ecs.components.timers.get_mut(&timer_e).unwrap();
        timer.update();
    }
}
