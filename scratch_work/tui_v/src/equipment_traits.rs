use crate::typeenums::*;

pub trait ItemTrait {
    fn item_symbol(&self) -> &'static str;
    fn item_name(&self) -> String;
}

impl Weapon {
    pub fn handedness(&self) -> i64 {
        match self {
            _ => 1,
        }
    }
}

impl ItemTrait for ItemType {
    fn item_symbol(&self) -> &'static str {
        match self {
            ItemType::Weapon(wep) => "/",

            ItemType::Clothing(cloth) => "t",
            ItemType::RangedWeapon(cloth) => ")",
            ItemType::Ammo(cloth) => "-",
        }
    }
    fn item_name(&self) -> String {
        match self {
            ItemType::Weapon(wep) => {
                format!("{wep}")
            }

            ItemType::Clothing(cloth) => {
                format!("{cloth}")
            }
            ItemType::RangedWeapon(cloth) => {
                format!("{cloth}")
            }
            ItemType::Ammo(cloth) => {
                format!("{cloth}")
            }
        }
    }
}
