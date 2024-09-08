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
            Weapon::Meč => 1,
            Weapon::Bulava => 2,
        }
    }
    fn damage_type(&self) -> DamageType {
        match self {
            Weapon::Meč => DamageType::Sharp,
            Weapon::Bulava => DamageType::Blunt,
        }
    }
}

impl WeaponTrait for RangedWeapon {
    fn damage(&self) -> i64 {
        match self {
            RangedWeapon::Lųk => 30,
            RangedWeapon::Pråšča => 20,
            RangedWeapon::Drotik => 10,
            RangedWeapon::Oščěp => 40,
        }
    }
    fn damage_type(&self) -> DamageType {
        match self {
            RangedWeapon::Lųk => DamageType::Sharp,
            RangedWeapon::Oščěp => DamageType::Sharp,
            RangedWeapon::Drotik => DamageType::Sharp,
            RangedWeapon::Pråšča => DamageType::Blunt,
        }
    }
}
