use crate::*;

pub enum DamageType {
    Sharp,
    Blunt,
}

pub trait MeleeWeaponTrait {
    fn damage(&self) -> i64;
    fn damage_type(&self) -> DamageType;
}

impl MeleeWeaponTrait for Weapon {
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
