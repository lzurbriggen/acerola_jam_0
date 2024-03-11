use crate::timer::Timer;

#[derive(Clone)]
pub enum WeaponType {
    Launcher,
    Balls,
    Dash,
}

pub enum Weapon {
    Launcher(Launcher),
    Balls(Balls),
    Dash(Dash),
}

pub struct Launcher {
    pub shoot_timer: Timer,
    pub damage: f32,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            shoot_timer: Timer::new(0.5, true),
            damage: 5.,
        }
    }
}

pub struct Balls {
    pub amount: usize,
    pub damage: f32,
}

impl Balls {
    pub fn new() -> Self {
        Self {
            amount: 2,
            damage: 5.,
        }
    }
}

pub struct Dash {}

impl Dash {
    pub fn new() -> Self {
        Self {}
    }
}
