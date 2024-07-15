use crate::*;

pub const MAP_SIZE: i64 = 100;

pub const WALL_FURNITURE: Furniture = Furniture::Wall(SolidMaterial::Wood(Tree::Glinos));
pub const TEGULA_ROOF: Roof = Roof::Tegula(SolidMaterial::Wood(Tree::Glinos));

pub const DIRT_FLOOR: Floor = Floor::Earth(EarthType::Dirt);
pub const SAND_FLOOR: Floor = Floor::Earth(EarthType::Sand);
pub const CLAY_FLOOR: Floor = Floor::Earth(EarthType::Clay);

pub const WATER_FLOOR: Floor = Floor::Liquid(LiquidType::Water);
