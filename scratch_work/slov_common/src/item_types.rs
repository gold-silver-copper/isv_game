use std::fmt::format;

use crate::*;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum SolidMaterial {
    Drěvo(WoodType),
    Metal(MetalType),
    Kamenj(StoneType),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum FabricMaterial {
    //vlakno vivsa tkanina plet'
    Koža(MammalType),
    Tkanina(PlantType),

    Lancuh(MetalType),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum PlantType {
    Drěvo(WoodType),
    Trava(GrassType),
    Kust(BushType),
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount)]
pub enum AnimalType {
    Mammal(MammalType),
    Fish(FishType),
    Bird(BirdType),
    Lizard(LizardType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimalPart {
    pub animal_type: AnimalType,
    pub animal_part: AnimalPartType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeleeWeapon {
    pub weapon_type: MeleeWeaponType,
    pub material_type: SolidMaterial,
}



#[derive(Clone, Debug, Display, PartialEq)]
pub enum ItemType {
    Melee(MeleeWeapon),
  
}


#[derive(Clone, Debug, Display, PartialEq)]
pub enum EntityType {
    Human(Human),
    Item(ItemType), //věć
    Monster(Animal),
    Mebelj(Mebelj),

    Råstlina(PlantType),

    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mebelj {
    pub mebelj_type: MebeljType,
    pub material: SolidMaterial,
}
