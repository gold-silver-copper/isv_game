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
#[derive(Clone, Debug, PartialEq)]
pub enum BodyPart {
    Head,
    HeadAccesory,
    TorsoUnder,
    TorsoMid,
    TorsoOver,
    Cloak,
    Waist,
    Legs,
    Feet,
    FeetOver,
}

impl Clothing {
    pub fn body_part_covered(&self) -> BodyPart {
        match self {
            Clothing::Toga => BodyPart::Cloak,
            Clothing::Helma => BodyPart::Head,
        }
    }
}

impl ItemTrait for ItemType {
    fn item_symbol(&self) -> &'static str {
        match self {
            ItemType::Weapon(wep) => "/",
            ItemType::Accessory(acc) => "c",

            ItemType::Clothing(cloth) => "t",
            ItemType::RangedWeapon(rang) => ")",
            ItemType::Ammo(amm) => "-",
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
            ItemType::Accessory(acc) => {
                format!("{acc}")
            }
            ItemType::RangedWeapon(rang) => {
                format!("{rang}")
            }
            ItemType::Ammo(amm) => {
                format!("{amm}")
            }
        }
    }
}
