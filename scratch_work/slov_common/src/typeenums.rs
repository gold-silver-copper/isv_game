use crate::*;



#[derive(Clone, Debug, Display, PartialEq)]
pub enum EarthType {
    Dirt,
    Clay,
    Sand,
}

#[derive(Clone, Debug, Display, PartialEq, Component)]
pub enum EntityType {
    Human,
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

