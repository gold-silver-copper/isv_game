use crate::*;

#[derive(Display, PartialEq, Clone)]
pub enum InputState {
    Basic,
    Inventory,
    RangedAttack,
}

pub enum ItemVecType {
    Inventory,
    Ground,
    Equipped,
}

impl Default for ItemVecType {
    fn default() -> Self {
        ItemVecType::Ground
    }
}

impl Default for InputState {
    fn default() -> Self {
        InputState::Basic
    }
}

pub enum Material {
    Metal(Metal),
    Wood(Tree),
}

// tegula imbrex
#[derive(Display)]
pub enum Roof {
    Tegula(Material),
}

pub enum Floor {
    Gravel(Material),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Glinos, //Maple Tree
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metal {
    Gold,
}

pub enum Furniture {
    Wall(Material),
}
