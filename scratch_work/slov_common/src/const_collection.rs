use crate::*;

pub const WALL_FURNITURE: Furniture = Furniture {
    name: "paries",
    symbol: "#",
    furniture_type: FurnitureType::Wall(SolidMaterial::Wood(Tree::Glinos)),
};

/*
dirt,brěst,solid, ,"166,112,78","166,112,78"
water,jasenj,liquid,~,"","4"
sand,lipa,granular, ,"233,225,194","233,225,194"
grass,jablånj,solid, ,"21,114,65","21,114,65"
eartt
    fg_color: RatColor::Rgb(145, 118, 83),
    bg_color: RatColor::Rgb(155, 118, 83),

water
     fg_color: RatColor::Rgb(35, 137, 218),
    bg_color: RatColor::Rgb(45, 117, 228),

*/
pub const DIRT_FLOOR: Floor = Floor::Earth(EarthType::Dirt);

pub const WATER_FLOOR: Floor = Floor::Liquid(LiquidType::Water);