use macroquad::prelude::rand;

use crate::{
    game_data::GameData,
    items::weapon::{Weapon, WeaponType},
};

pub struct UpgradeDescription {
    pub texture_name: String,
    pub text: Vec<String>,
}

impl UpgradeDescription {
    pub fn new(texture_name: &str, line1: &str) -> Self {
        Self {
            texture_name: texture_name.to_string(),
            text: vec![line1.to_string()],
        }
    }

    pub fn new_with_line2(texture_name: &str, line1: &str, line2: &str) -> Self {
        Self {
            texture_name: texture_name.to_string(),
            text: vec![line1.to_string(), line2.to_string()],
        }
    }
}

#[derive(Clone)]
pub enum Upgrade {
    Item(ItemUpgrade),
    Weapon(WeaponType),
    WeaponUpgrade(WeaponUpgrade),
    CommonUpgrade(CommonUpgrade),
}

impl Upgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            Upgrade::Item(item) => item.description(),
            Upgrade::Weapon(weapon) => match weapon {
                WeaponType::Launcher => UpgradeDescription::new("upgrade_launcher", "Launcher"),
                WeaponType::Balls => UpgradeDescription::new("upgrade_balls", "Balls"),
                WeaponType::Dash => UpgradeDescription::new("upgrade_dash", "Dash"),
            },
            Upgrade::WeaponUpgrade(upgrade) => upgrade.description(),
            Upgrade::CommonUpgrade(upgrade) => upgrade.description(),
        }
    }
}

#[derive(Clone)]
pub enum ItemUpgrade {
    Hp(f32),
    AnomalyBig,
    AnomalySmall,
}

impl ItemUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            ItemUpgrade::Hp(hp) => {
                UpgradeDescription::new("upgrade_item_hp", format!("+ {} HP", hp).as_str())
            }
            ItemUpgrade::AnomalyBig => {
                UpgradeDescription::new_with_line2("upgrade_item_anomaly_big", "- 10%", "Anomaly")
            }
            ItemUpgrade::AnomalySmall => {
                UpgradeDescription::new_with_line2("upgrade_item_anomaly_small", "- 2%", "Anomaly")
            }
        }
    }
}

#[derive(Clone)]
pub enum CommonUpgrade {
    MaxHp(u8),
    MoveSpeed(f32),
    ItemDropChance(i32),
}

impl CommonUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            CommonUpgrade::MaxHp(hp) => UpgradeDescription::new_with_line2(
                "upgrade_common_max_hp",
                format!("+ {}", hp).as_str(),
                "Max HP",
            ),
            CommonUpgrade::MoveSpeed(speed) => UpgradeDescription::new_with_line2(
                "upgrade_move_speed",
                format!("+ {:.0}%", speed * 100.).as_str(),
                "Speed",
            ),
            CommonUpgrade::ItemDropChance(_) => {
                UpgradeDescription::new_with_line2("upgrade_items", "Inc. Item", "Drops")
            }
        }
    }
}

#[derive(Clone)]
pub enum WeaponUpgrade {
    Launcher(LauncherUpgrade),
    Balls(BallsUpgrade),
    Dash(DashUpgrade),
}

impl WeaponUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            WeaponUpgrade::Launcher(upgrade) => upgrade.description(),
            WeaponUpgrade::Balls(upgrade) => upgrade.description(),
            WeaponUpgrade::Dash(upgrade) => upgrade.description(),
        }
    }
}

#[derive(Clone)]
pub enum LauncherUpgrade {
    FireRate(f32),
    Damage(f32),
}

#[derive(Clone)]
pub enum BallsUpgrade {
    Amount(usize),
    Damage(f32),
    RotateSpeed(f32),
}

#[derive(Clone)]
pub enum DashUpgrade {
    Damage(f32),
    TimerDecrease(f32),
}

impl LauncherUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            LauncherUpgrade::FireRate(rate) => UpgradeDescription::new_with_line2(
                "upgrade_launcher",
                format!("+ {:.0}%", rate * 100.).as_str(),
                "Fire Rate",
            ),
            LauncherUpgrade::Damage(dmg) => UpgradeDescription::new_with_line2(
                "upgrade_launcher",
                format!("+ {:.0}", dmg).as_str(),
                "Damage",
            ),
        }
    }
}

impl BallsUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            BallsUpgrade::Amount(amount) => UpgradeDescription::new_with_line2(
                "upgrade_balls",
                format!("+ {}", amount).as_str(),
                "Ball",
            ),
            BallsUpgrade::Damage(dmg) => UpgradeDescription::new_with_line2(
                "upgrade_balls",
                format!("+ {:.0}", dmg).as_str(),
                "Damage",
            ),
            BallsUpgrade::RotateSpeed(speed) => UpgradeDescription::new_with_line2(
                "upgrade_balls",
                format!("+ {:.0}%", speed * 100.).as_str(),
                "Rot. Speed",
            ),
        }
    }
}

impl DashUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        // TODO

        match self {
            DashUpgrade::Damage(dmg) => UpgradeDescription::new_with_line2(
                "upgrade_dash",
                format!("+ {:.0}", dmg).as_str(),
                "Damage",
            ),
            DashUpgrade::TimerDecrease(decrease) => UpgradeDescription::new_with_line2(
                "upgrade_dash",
                format!("+ {:.0}%", decrease * 100.).as_str(),
                "Timer dec.",
            ),
        }
    }
}

pub fn generate_upgrade(data: &GameData) -> Vec<Upgrade> {
    let mut upgrades: Vec<Upgrade> = vec![];

    // for i in 0..3 {
    //     loop {
    //         let upgrade_index = rand::gen_range(0, Upgrade::len());
    //         let upgrade = Upgrade::generate_by_index(upgrade_index);
    //         if i < 2
    //             && upgrades.iter().all(|u| u.is_general_upgrade())
    //             && upgrade.is_general_upgrade()
    //         {
    //             continue;
    //         }
    //         // TODO: match weapon
    //         upgrades.push(upgrade);
    //     }
    // }

    upgrades
}

pub struct Upgrades {
    item_upgrades: Vec<ItemUpgrade>,
    common_upgrades: Vec<CommonUpgrade>,
    launcher_upgrades: Vec<LauncherUpgrade>,
    balls_upgrades: Vec<BallsUpgrade>,
    dash_upgrades: Vec<DashUpgrade>,
}

impl Upgrades {
    pub fn new() -> Self {
        Self {
            item_upgrades: vec![
                ItemUpgrade::Hp(1.),
                ItemUpgrade::AnomalyBig,
                ItemUpgrade::AnomalySmall,
            ],
            common_upgrades: vec![
                CommonUpgrade::MaxHp(1),
                CommonUpgrade::MoveSpeed(0.1),
                CommonUpgrade::ItemDropChance(1),
            ],
            launcher_upgrades: vec![LauncherUpgrade::FireRate(0.1), LauncherUpgrade::Damage(5.)],
            balls_upgrades: vec![
                BallsUpgrade::Amount(1),
                BallsUpgrade::Damage(5.),
                BallsUpgrade::RotateSpeed(0.15),
            ],
            dash_upgrades: vec![DashUpgrade::Damage(5.), DashUpgrade::TimerDecrease(0.15)],
        }
    }

    pub fn generate_upgrades(
        &self,
        weapon: &Weapon,
        missing_hp: f32,
        aberration: f32,
    ) -> Vec<Upgrade> {
        let weapon_upgrade = match weapon {
            Weapon::Launcher(_) => Upgrade::WeaponUpgrade(WeaponUpgrade::Launcher(
                self.launcher_upgrades[rand::gen_range(0, self.launcher_upgrades.len())].clone(),
            )),
            Weapon::Balls(_) => Upgrade::WeaponUpgrade(WeaponUpgrade::Balls(
                self.balls_upgrades[rand::gen_range(0, self.balls_upgrades.len())].clone(),
            )),
            Weapon::Dash(_) => Upgrade::WeaponUpgrade(WeaponUpgrade::Dash(
                self.dash_upgrades[rand::gen_range(0, self.dash_upgrades.len())].clone(),
            )),
        };

        let mut upgrades = vec![weapon_upgrade];
        while upgrades.len() < 3 {
            let collection_index = rand::gen_range(0, 2);
            match collection_index {
                0 => {
                    let upgrade =
                        self.item_upgrades[rand::gen_range(0, self.item_upgrades.len())].clone();
                    if let ItemUpgrade::Hp(hp) = upgrade {
                        if hp > missing_hp {
                            continue;
                        }
                    }
                    upgrades.push(Upgrade::Item(upgrade));
                }
                1 => {
                    let upgrade = self.common_upgrades
                        [rand::gen_range(0, self.common_upgrades.len())]
                    .clone();
                    upgrades.push(Upgrade::CommonUpgrade(upgrade));
                }
                _ => unreachable!(),
            }
        }

        upgrades
        // vec![
        //     weapon_upgrade,
        //     // Upgrade::Weapon(WeaponType::Dash),
        //     // Upgrade::WeaponUpgrade(WeaponUpgrade::Launcher(LauncherUpgrade::(5.))),
        //     Upgrade::WeaponUpgrade(WeaponUpgrade::Launcher(LauncherUpgrade::Damage(5.))),
        //     Upgrade::WeaponUpgrade(WeaponUpgrade::Launcher(LauncherUpgrade::FireRate(0.1))),
        //     // Upgrade::WeaponUpgrade(WeaponUpgrade::Dash(DashUpgrade::Damage(10.))),
        //     Upgrade::CommonUpgrade(CommonUpgrade::MaxHp(1)),
        //     // Upgrade::CommonUpgrade(CommonUpgrade::MoveSpeed(0.10)),
        //     // Upgrade::CommonUpgrade(CommonUpgrade::ItemDropChance(1)),
        //     // Upgrade::Item(ItemUpgrade::Hp(1.)),
        //     // Upgrade::WeaponUpgrade(WeaponUpgrade::Shooter(ShooterUpgrade::FireRate(15.))),
        // ]
    }

    pub fn weapon_selection() -> Vec<Upgrade> {
        vec![
            Upgrade::Weapon(WeaponType::Launcher),
            Upgrade::Weapon(WeaponType::Balls),
            Upgrade::Weapon(WeaponType::Dash),
        ]
    }
}
