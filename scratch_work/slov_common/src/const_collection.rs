use crate::*;

pub const WALL_FURNITURE: Furniture =  Furniture::Wall(SolidMaterial::Wood(Tree::Glinos));



pub const DIRT_FLOOR: Floor = Floor::Earth(EarthType::Dirt);

pub const WATER_FLOOR: Floor = Floor::Liquid(LiquidType::Water);