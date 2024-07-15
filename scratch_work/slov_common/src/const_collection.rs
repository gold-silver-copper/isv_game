use crate::*;

pub const MAP_SIZE: i64 = 1000;

pub const WALL_FURNITURE: Furniture = Furniture::Wall(SolidMaterial::Wood(Tree::Glinos));

pub const DIRT_FLOOR: Floor = Floor::Earth(EarthType::Dirt);
pub const SAND_FLOOR: Floor = Floor::Earth(EarthType::Sand);
pub const CLAY_FLOOR: Floor = Floor::Earth(EarthType::Clay);

pub const WATER_FLOOR: Floor = Floor::Liquid(LiquidType::Water);
