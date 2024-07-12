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







#[derive(Clone, Debug, Display, PartialEq)]
pub enum EntityType {
    Human(Human),
   
   
    Mebelj(Mebelj),

    Råstlina(PlantType),

    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mebelj {
    pub mebelj_type: MebeljType,
    pub material: SolidMaterial,
}
