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
   
    Tkanina(PlantType),

    Lancuh(MetalType),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum PlantType {
    Drěvo(WoodType),
    Trava(GrassType),
    Kust(BushType),
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
   
    Mebelj(Mebelj),

    Råstlina(PlantType),

    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mebelj {
    pub mebelj_type: MebeljType,
    pub material: SolidMaterial,
}
