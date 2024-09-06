use crate::*;

pub enum DamageType {
    Sharp,
    Blunt,
}

pub trait WeaponTrait {
    fn damage(&self) -> i64;
    fn damage_type(&self) -> DamageType;
}

impl WeaponTrait for Weapon {
    fn damage(&self) -> i64 {
        match self {
            Weapon::Sword => 1,
            Weapon::Mace => 2,
        }
    }
    fn damage_type(&self) -> DamageType {
        match self {
            Weapon::Sword => DamageType::Sharp,
            Weapon::Mace => DamageType::Blunt,
        }
    }
}

impl WeaponTrait for RangedWeapon {
    fn damage(&self) -> i64 {
        match self {
            RangedWeapon::Šuk => 30,
            RangedWeapon::Pråšča => 1,
        }
    }
    fn damage_type(&self) -> DamageType {
        match self {
            RangedWeapon::Šuk => DamageType::Sharp,
            RangedWeapon::Pråšča => DamageType::Blunt,
        }
    }
}
