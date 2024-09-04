use crate::first_char;
use crate::typeenums::*;

pub trait ItemTrait {
    fn item_symbol(&self) -> String;
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
    fn item_symbol(&self) -> String {
        let nameik = self.item_name();
        let a = first_char(&nameik);
        a.to_lowercase()
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
