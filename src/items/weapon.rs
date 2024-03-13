use std::f32::consts::TAU;

use macroquad::{math::Vec2, time::get_frame_time};

use crate::{
    entity::upgrades::{BallsUpgrade, DashUpgrade, LauncherUpgrade},
    timer::Timer,
};

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
    pub base_timer_duration: f32,
    pub base_damage: f32,
    pub upgrades: Vec<LauncherUpgrade>,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            shoot_timer: Timer::new(0.25, false),
            base_timer_duration: 0.25,
            base_damage: 50.,
            upgrades: vec![],
        }
    }

    pub fn get_upgraded_data(&self) -> LauncherData {
        let mut damage = self.base_damage;

        let mut duration_percentage_decrease = 0.;
        for upgrade in &self.upgrades {
            match upgrade {
                LauncherUpgrade::FireRate(rate) => {
                    duration_percentage_decrease += rate;
                }
                LauncherUpgrade::Damage(dmg) => damage += dmg,
            }
        }

        LauncherData {
            damage,
            timer_duration: self.base_timer_duration * (1. - duration_percentage_decrease),
        }
    }
}

pub struct LauncherData {
    pub damage: f32,
    pub timer_duration: f32,
}

pub struct Balls {
    pub ball_spawn_timer: Timer,
    pub base_amount: usize,
    pub base_damage: f32,
    pub rotation_progress: f32,
    pub base_rotation_speed: f32,
    pub buffered_spawns: usize,
    pub upgrades: Vec<BallsUpgrade>,
}

pub struct BallsData {
    pub amount: usize,
    pub damage: f32,
    pub rotation_speed: f32,
}

impl Balls {
    pub fn new() -> Self {
        Self {
            ball_spawn_timer: Timer::new(0.4, true),
            base_amount: 3,
            base_damage: 7.,
            rotation_progress: 0.,
            upgrades: vec![],
            base_rotation_speed: 0.15,
            buffered_spawns: 3,
        }
    }

    pub fn update(&mut self) {
        let data = self.get_upgraded_data();
        self.rotation_progress += get_frame_time() * (TAU * data.rotation_speed);
        self.ball_spawn_timer.update();
        if self.ball_spawn_timer.just_completed() {
            self.buffered_spawns = (self.buffered_spawns + 1).min(data.amount);
        }
    }

    pub fn get_upgraded_data(&self) -> BallsData {
        let mut amount = self.base_amount;
        let mut damage = self.base_damage;
        let mut rotation_speed_percentage = 0.;

        for upgrade in &self.upgrades {
            match upgrade {
                BallsUpgrade::Amount(added_amount) => amount += added_amount,
                BallsUpgrade::Damage(dmg) => damage += dmg,
                BallsUpgrade::RotateSpeed(speed) => rotation_speed_percentage += speed,
            }
        }

        BallsData {
            amount,
            damage,
            rotation_speed: self.base_rotation_speed * (1. + rotation_speed_percentage),
        }
    }
}

pub struct Dash {
    pub dash_timer: Timer,
    pub dashing_timer: Timer,
    pub speed: f32,
    pub dashing: bool,
    pub direction: Vec2,
    pub shadow_timer: Timer,
    pub shadow_index: usize,
    pub upgrades: Vec<DashUpgrade>,
    pub base_damage: f32,
    pub base_dash_timer_duration: f32,
}

pub struct DashData {
    pub damage: f32,
    pub dash_timer_duration: f32,
}

impl Dash {
    pub fn new() -> Self {
        let dash_timer_duration = 0.6;
        Self {
            dash_timer: Timer::new(dash_timer_duration, false),
            base_dash_timer_duration: dash_timer_duration,
            dashing_timer: Timer::new(0.24, false),
            shadow_timer: Timer::new(0.08, false),
            speed: 150.,
            dashing: false,
            direction: Vec2::X,
            shadow_index: 0,
            upgrades: vec![],
            base_damage: 12.,
        }
    }

    pub fn update(&mut self) {
        self.dash_timer.update();
        self.dashing_timer.update();
        self.shadow_timer.update();
    }

    pub fn get_upgraded_data(&self) -> DashData {
        let mut damage = self.base_damage;

        let mut duration_percentage_decrease = 0.;
        for upgrade in &self.upgrades {
            match upgrade {
                DashUpgrade::Damage(increase) => damage += increase,
                DashUpgrade::TimerDecrease(decrease) => duration_percentage_decrease += decrease,
            }
        }

        DashData {
            damage,
            dash_timer_duration: self.base_dash_timer_duration
                * (1. - duration_percentage_decrease),
        }
    }
}
