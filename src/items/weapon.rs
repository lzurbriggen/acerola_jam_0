use std::f32::consts::TAU;

use macroquad::{math::Vec2, time::get_frame_time};

use crate::{
    entity::upgrades::{BallsUpgrade, Upgrade},
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
    pub damage: f32,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            shoot_timer: Timer::new(0.25, false),
            damage: 5.,
        }
    }
}

pub struct Balls {
    pub ball_spawn_timer: Timer,
    pub base_amount: usize,
    pub base_damage: f32,
    pub rotation_progress: f32,
    pub upgrades: Vec<BallsUpgrade>,
    pub base_rotation_speed: f32,
    pub buffered_spawns: usize,
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
            base_damage: 5.,
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
        let mut rotation_speed = self.base_rotation_speed;

        for upgrade in &self.upgrades {
            match upgrade {
                BallsUpgrade::Amount(added_amount) => amount += added_amount,
            }
        }

        BallsData {
            amount,
            damage,
            rotation_speed,
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
}

impl Dash {
    pub fn new() -> Self {
        Self {
            dash_timer: Timer::new(0.6, false),
            dashing_timer: Timer::new(0.24, false),
            shadow_timer: Timer::new(0.08, false),
            speed: 150.,
            dashing: false,
            direction: Vec2::X,
            shadow_index: 0,
        }
    }

    pub fn update(&mut self) {
        self.dash_timer.update();
        self.dashing_timer.update();
        self.shadow_timer.update();
    }
}
