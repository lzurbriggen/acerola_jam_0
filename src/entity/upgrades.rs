use crate::{game_data::GameData, items::weapon::WeaponType};

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
                WeaponType::Balls => UpgradeDescription::new("upgrade_launcher", "Balls"),
                WeaponType::Dash => UpgradeDescription::new("upgrade_launcher", "Dash"),
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
            ItemUpgrade::AnomalyBig => UpgradeDescription::new_with_line2(
                "upgrade_item_anomaly_big",
                "- 50%",
                "Aberration",
            ),
            ItemUpgrade::AnomalySmall => UpgradeDescription::new_with_line2(
                "upgrade_item_anomaly_small",
                "- 10%",
                "Aberration",
            ),
        }
    }
}

#[derive(Clone)]
pub enum CommonUpgrade {
    MaxHp(u8),
    MoveSpeed(f32),
}

impl CommonUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            CommonUpgrade::MaxHp(hp) => UpgradeDescription::new_with_line2(
                "upgrade_common_max_hp",
                format!("+ {}", hp).as_str(),
                "Max HP",
            ),
            CommonUpgrade::MoveSpeed(speed) => {
                // TODO: texture
                UpgradeDescription::new_with_line2(
                    "upgrade_common_max_hp",
                    format!("+ {}%", speed).as_str(),
                    "Move Speed",
                )
            }
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum ShooterUpgrade {
    FireRate(f32),
}

#[derive(Clone)]
pub enum BallsUpgrade {
    Amount(usize),
}

#[derive(Clone)]
pub enum DashUpgrade {}

impl ShooterUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        match self {
            ShooterUpgrade::FireRate(rate) => UpgradeDescription::new_with_line2(
                "upgrade_launcher",
                format!("+ {}%", rate).as_str(),
                "Fire Rate",
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
                "Projectiles",
            ),
        }
    }
}

impl DashUpgrade {
    pub fn description(&self) -> UpgradeDescription {
        // TODO
        UpgradeDescription::new("upgrade_dash", "")
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
    shooter_upgrades: Vec<ShooterUpgrade>,
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
            common_upgrades: vec![CommonUpgrade::MaxHp(1), CommonUpgrade::MoveSpeed(10.)],
            shooter_upgrades: vec![],
            balls_upgrades: vec![],
            dash_upgrades: vec![],
        }
    }

    pub fn weapon_selection() -> Vec<Upgrade> {
        vec![
            Upgrade::CommonUpgrade(CommonUpgrade::MaxHp(1)),
            // Upgrade::Weapon(WeaponType::Launcher),
            // Upgrade::Weapon(WeaponType::Balls),
            Upgrade::Item(ItemUpgrade::Hp(1.)),
            // Upgrade::Weapon(WeaponType::Dash),
            Upgrade::WeaponUpgrade(WeaponUpgrade::Shooter(ShooterUpgrade::FireRate(15.))),
        ]
    }
}
