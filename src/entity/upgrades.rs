use macroquad::prelude::rand;

use crate::{game_data::GameData, items::weapon::Weapon};

pub struct UpgradeDescription {
    texture_name: String,
    text_line1: String,
    text_line2: Option<String>,
}

impl UpgradeDescription {
    pub fn new(texture_name: &str, line1: &str) -> Self {
        Self {
            texture_name: texture_name.to_string(),
            text_line1: line1.to_string(),
            text_line2: None,
        }
    }

    pub fn new_with_line2(texture_name: &str, line1: &str, line2: &str) -> Self {
        Self {
            texture_name: texture_name.to_string(),
            text_line1: line1.to_string(),
            text_line2: Some(line2.to_string()),
        }
    }
}

pub enum Upgrade {
    Item(ItemUpgrade),
    Weapon(Weapon),
    WeaponUpgrade(WeaponUpgrade),
    CommonUpgrade(CommonUpgrade),
}

impl Upgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            Upgrade::Item(item) => item.description(),
            Upgrade::Weapon(weapon) => match weapon {
                Weapon::Shooter(shooter) => UpgradeDescription::new("Launcher"),
            },
            Upgrade::WeaponUpgrade(upgrade) => upgrade.description(),
            Upgrade::CommonUpgrade(upgrade) => upgrade.description(),
        }
    }
}

pub enum ItemUpgrade {
    Hp(f32),
    AnomalyBig,
    AnomalySmall,
}

impl ItemUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            ItemUpgrade::Hp(hp) => UpgradeDescription::new(format!("+ {} HP", hp).as_str()),
            ItemUpgrade::AnomalyBig => UpgradeDescription::new_with_line2("- 50%", "Aberration"),
            ItemUpgrade::AnomalySmall => UpgradeDescription::new_with_line2("- 10%", "Aberration"),
        }
    }
}

pub enum CommonUpgrade {
    MaxHp(u8),
    MoveSpeed(f32),
}

impl CommonUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            CommonUpgrade::MaxHp(hp) => {
                UpgradeDescription::new_with_line2(format!("+ {}", hp).as_str(), "Max HP")
            }
            CommonUpgrade::MoveSpeed(speed) => {
                UpgradeDescription::new_with_line2(format!("+ {}%", speed).as_str(), "Move Speed")
            }
        }
    }
}

pub enum WeaponUpgrade {
    Shooter(ShooterUpgrade),
    Balls(BallsUpgrade),
    Dash(DashUpgrade),
}

impl WeaponUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            WeaponUpgrade::Shooter(upgrade) => upgrade.description(),
            WeaponUpgrade::Balls(upgrade) => upgrade.description(),
            WeaponUpgrade::Dash(upgrade) => upgrade.description(),
        }
    }
}

pub enum ShooterUpgrade {
    FireRate(f32),
}

pub enum BallsUpgrade {
    Amount(usize),
}

pub enum DashUpgrade {}

impl ShooterUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            ShooterUpgrade::FireRate(rate) => {
                UpgradeDescription::new_with_line2(format!("+ {}%", rate).as_str(), "Fire Rate")
            }
        }
    }
}

impl BallsUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            BallsUpgrade::Amount(amount) => {
                UpgradeDescription::new_with_line2(format!("+ {}", amount).as_str(), "Projectiles")
            }
        }
    }
}

impl DashUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        // TODO
        UpgradeDescription::new("")
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
