use crate::entity::{
    entities::{Enemy, Entities},
    traits::TimerProgress,
};

pub fn update_timers(entities: &mut Entities) {
    update_timer(&mut entities.player);
    for enemy in &mut entities.enemies {
        match enemy {
            Enemy::Hopper(hopper) => update_timer(hopper),
        }
    }
}

fn update_timer<T: TimerProgress>(timed: &mut T) {
    timed.update_timers()
}
