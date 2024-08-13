use crate::*;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum EarthType {
    Dirt,
    Clay,
    Sand,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum EntityType {
    Animalia,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ItemType {
    Clothing,
    Weapon,
    Consumable,
    Armor,
}

impl EntityType {
    pub fn to_complex_noun(&self) -> ComplexNoun {
        match self {
            _ => {
                let head_noun = format!("{}", self).to_lowercase();
                //println!("{}",head_noun);
                ComplexNoun {
                    head_noun,
                    ..ComplexNoun::default()
                }
            }
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Roof {
    Tegula(SolidMaterial),
    Imbrex(SolidMaterial),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SolidMaterial {
    Wood(Tree),
    Stone(Mineral),
    Metal(Metal),
    Terracotta(EarthType),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Glinos, //Maple Tree
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mineral {
    Iaspis, //Gold
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metal {
    Glinos, //Maple Tree
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Furniture {
    Wall(SolidMaterial),
    Door(SolidMaterial),
    Trinket,
}
