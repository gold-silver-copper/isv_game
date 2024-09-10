use crate::first_char;
use crate::typeenums::*;
use crate::CoordinateUnit;

pub trait ItemTrait {
    fn item_symbol(&self) -> String;
    fn item_name(&self) -> String;
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

pub trait Armor {
    fn body_part_covered(&self) -> BodyPart;
    fn defense_value(&self) -> i32;
}

impl Armor for Clothing {
    fn body_part_covered(&self) -> BodyPart {
        match self {
            Clothing::Toga => BodyPart::Cloak,
            Clothing::Helma => BodyPart::Head,
        }
    }
    fn defense_value(&self) -> i32 {
        match self {
            Clothing::Toga => 5,
            Clothing::Helma => 3,
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

            ItemType::Ammo(amm) => match amm {
                Ammo::Kulja(x) | Ammo::Strěla(x) | Ammo::Oščěp(x) | Ammo::Drotik(x) => {
                    format!("{amm}({x})")
                }
            },
        }
    }
}

pub enum DamageType {
    Sharp,
    Blunt,
}
impl Weapon {
    pub fn handedness(&self) -> i32 {
        match self {
            _ => 1,
        }
    }
}
pub trait WeaponTrait {
    fn damage(&self) -> i32;
}

impl WeaponTrait for Weapon {
    fn damage(&self) -> i32 {
        match self {
            Weapon::Meč => 1,
            Weapon::Bulava => 2,
        }
    }
}
impl RangedWeapon {
    pub fn ideal_range(&self) -> CoordinateUnit {
        match self {
            RangedWeapon::Lųk => 10,
            RangedWeapon::Pråšča => 7,
            RangedWeapon::Drotik => 3,
            RangedWeapon::Oščěp => 5,
        }
    }
}

impl WeaponTrait for RangedWeapon {
    fn damage(&self) -> i32 {
        match self {
            RangedWeapon::Lųk => 30,
            RangedWeapon::Pråšča => 20,
            RangedWeapon::Drotik => 10,
            RangedWeapon::Oščěp => 40,
        }
    }
}
