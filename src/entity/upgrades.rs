use macroquad::prelude::rand;

use crate::game_data::GameData;

pub enum Upgrade {
    MaxHp(u8),
    Shooter(ShooterUpgrade),
    MoveSpeed(f32),
}

impl Upgrade {
    pub fn description(&self) -> String {
        match self {
            Upgrade::MaxHp(hp) => format!("+ {} HP", hp),
            Upgrade::Shooter(upgrade) => upgrade.description(),
            Upgrade::MoveSpeed(speed) => format!("+ {} Move Speed", speed),
        }
    }

    pub fn is_general_upgrade(&self) -> bool {
        match self {
            Upgrade::MaxHp(_) => true,
            Upgrade::MoveSpeed(_) => true,
            _ => false,
        }
    }

    pub fn is_shoot_upgrade(&self) -> bool {
        match self {
            Upgrade::Shooter(_) => true,
            _ => false,
        }
    }

    pub fn len() -> usize {
        3
    }

    pub fn generate_by_index(index: usize) -> Self {
        match index {
            0 => Upgrade::MaxHp(1),
            1 => Upgrade::MoveSpeed(10.),
            2 => Upgrade::Shooter(ShooterUpgrade::generate_by_index(index)),
            _ => unreachable!(),
        }
    }
}

pub enum ShooterUpgrade {
    FireRate(f32),
}

impl ShooterUpgrade {
    pub fn description(&self) -> String {
        match self {
            ShooterUpgrade::FireRate(rate) => format!("+ {} Fire Rate", rate),
        }
    }

    pub fn len() -> usize {
        1
    }

    pub fn generate_by_index(index: usize) -> Self {
        match index {
            0 => ShooterUpgrade::FireRate(5.),
            _ => unreachable!(),
        }
    }
}

pub fn generate_upgrade(data: &GameData) -> Vec<Upgrade> {
    let mut upgrades: Vec<Upgrade> = vec![];

    for i in 0..3 {
        loop {
            let upgrade_index = rand::gen_range(0, Upgrade::len());
            let upgrade = Upgrade::generate_by_index(upgrade_index);
            if i < 2
                && upgrades.iter().all(|u| u.is_general_upgrade())
                && upgrade.is_general_upgrade()
            {
                continue;
            }
            // TODO: match weapon
            upgrades.push(upgrade);
        }
    }

    upgrades
}
