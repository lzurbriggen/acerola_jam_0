use crate::timer::Timer;

pub enum Weapon {
    Shooter(Shooter),
}

pub struct Shooter {
    pub shoot_timer: Timer,
    pub damage: f32,
}

impl Shooter {
    pub fn new() -> Self {
        Self {
            shoot_timer: Timer::new(1.5, true),
            damage: 5.,
        }
    }
}
