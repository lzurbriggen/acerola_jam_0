use crate::timer::Timer;

#[derive(Clone)]
pub enum WeaponType {
    Launcher,
    Balls,
    Dash,
}

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
            shoot_timer: Timer::new(0.5, true),
            damage: 5.,
        }
    }
}
