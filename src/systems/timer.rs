use crate::entity::{entities::Components, entity_id::Entity};

pub fn update_timers(entities: &Vec<Entity>, components: &mut Components) {
    let timers = entities
        .iter()
        .filter(|e| components.timers.contains_key(e))
        .collect::<Vec<&Entity>>();

    for timer_e in &timers {
        let timer = components.timers.get_mut(&timer_e).unwrap();
        timer.update();
    }
}
